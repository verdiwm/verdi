use std::{
    io::{self, IoSlice, IoSliceMut},
    mem::MaybeUninit,
    os::{
        fd::RawFd,
        unix::net::{AncillaryData, SocketAncillary, UnixStream},
    },
    pin::Pin,
    task::{Context, Poll},
};

use bytes::{Buf, BufMut, BytesMut};
use futures_util::{ready, Sink, Stream};
use pin_project_lite::pin_project;
use tokio::io::{unix::AsyncFd, ReadBuf};
use tracing::trace;

use super::{DecodeError, Message};

pin_project! {
    pub struct Socket {
        stream: AsyncFd<UnixStream>,
        codec: MessageCodec,
        read_state: ReadState,
        write_state: WriteState,
    }
}

#[derive(Debug)]
struct MessageCodec;

impl MessageCodec {
    const fn new() -> Self {
        Self {}
    }

    fn decode(
        &mut self,
        src: &mut BytesMut,
        fds: &mut Vec<RawFd>,
    ) -> std::result::Result<Option<Message>, DecodeError> {
        if src.is_empty() {
            return Ok(None);
        }

        Message::from_bytes(src, fds).map(Option::Some)
    }

    fn decode_eof(
        &mut self,
        buf: &mut BytesMut,
        fds: &mut Vec<RawFd>,
    ) -> Result<Option<Message>, DecodeError> {
        match self.decode(buf, fds)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "bytes remaining on stream").into())
                }
            }
        }
    }
}

struct ReadState {
    eof: bool,
    buffer: BytesMut,
    fds: Vec<RawFd>,
    is_readable: bool,
    has_errored: bool,
}

impl ReadState {
    pub fn new() -> Self {
        Self {
            eof: false,
            buffer: BytesMut::with_capacity(8192),
            fds: Vec::new(),
            is_readable: false,
            has_errored: false,
        }
    }
}

struct WriteState {
    buffer: BytesMut,
    fds: Vec<RawFd>,
}

impl WriteState {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::with_capacity(8192),
            fds: Vec::new(),
        }
    }
}

impl Stream for Socket {
    type Item = Result<Message, DecodeError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let pinned = self.project();
        let state = pinned.read_state;

        loop {
            if state.has_errored {
                // preparing has_errored -> paused
                trace!("Returning None and setting paused");
                state.is_readable = false;
                state.has_errored = false;
                return Poll::Ready(None);
            }

            // Repeatedly call `decode` or `decode_eof` while the buffer is "readable",
            // i.e. it _might_ contain data consumable as a frame or closing frame.
            // Both signal that there is no such data by returning `None`.
            //
            // If `decode` couldn't read a frame and the upstream source has returned eof,
            // `decode_eof` will attempt to decode the remaining bytes as closing frames.
            //
            // If the underlying AsyncRead is resumable, we may continue after an EOF,
            // but must finish emitting all of it's associated `decode_eof` frames.
            // Furthermore, we don't want to emit any `decode_eof` frames on retried
            // reads after an EOF unless we've actually read more data.
            if state.is_readable {
                // pausing or framing
                if state.eof {
                    // pausing
                    let frame = pinned
                        .codec
                        .decode_eof(&mut state.buffer, &mut state.fds)
                        .map_err(|err| {
                            trace!("Got an error, going to errored state");
                            state.has_errored = true;
                            err
                        })?;
                    if frame.is_none() {
                        state.is_readable = false; // prepare pausing -> paused
                    }
                    // implicit pausing -> pausing or pausing -> paused
                    return Poll::Ready(frame.map(Ok));
                }

                // framing
                trace!("attempting to decode a frame");

                if let Some(frame) = pinned
                    .codec
                    .decode(&mut state.buffer, &mut state.fds)
                    .map_err(|op| {
                        trace!("Got an error, going to errored state");
                        state.has_errored = true;
                        op
                    })?
                {
                    trace!("frame decoded from buffer");
                    // implicit framing -> framing
                    return Poll::Ready(Some(Ok(frame)));
                }

                // framing -> reading
                state.is_readable = false;
            }
            // reading or paused
            // If we can't build a frame yet, try to read more data and try again.
            // Make sure we've got room for at least one byte to read to ensure
            // that we don't get a spurious 0 that looks like EOF.
            state.buffer.reserve(1);
            #[allow(clippy::blocks_in_conditions)]
            let bytect =
                match Self::poll_read_buf(pinned.stream, cx, &mut state.buffer, &mut state.fds)
                    .map_err(|err| {
                        trace!("Got an error, going to errored state");
                        state.has_errored = true;
                        err
                    })? {
                    Poll::Ready(ct) => ct,
                    // implicit reading -> reading or implicit paused -> paused
                    Poll::Pending => return Poll::Pending,
                };
            if bytect == 0 {
                if state.eof {
                    // We're already at an EOF, and since we've reached this path
                    // we're also not readable. This implies that we've already finished
                    // our `decode_eof` handling, so we can simply return `None`.
                    // implicit paused -> paused
                    return Poll::Ready(None);
                }
                // prepare reading -> paused
                state.eof = true;
            } else {
                // prepare paused -> framing or noop reading -> framing
                state.eof = false;
            }

            // paused -> framing or reading -> framing or reading -> pausing
            state.is_readable = true;
        }
    }
}

impl Sink<Message> for Socket {
    type Error = io::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.stream.poll_write_ready(cx) {
            Poll::Ready(_) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
        }
    }

    fn start_send(self: std::pin::Pin<&mut Self>, message: Message) -> Result<(), Self::Error> {
        let pinned = self.project();
        let state = pinned.write_state;

        message.to_bytes(&mut state.buffer, &mut state.fds);

        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        const MAX_BUFS: usize = 64;

        let pinned = self.project();
        let state = pinned.write_state;

        let mut ancillary_buffer = [0; 128];
        let mut ancillary = SocketAncillary::new(&mut ancillary_buffer);

        ancillary.add_fds(&state.fds);

        while !state.buffer.is_empty() {
            let mut guard = ready!(pinned.stream.poll_write_ready(cx))?;

            let mut slices = [IoSlice::new(&[]); MAX_BUFS];
            let cnt = state.buffer.chunks_vectored(&mut slices);

            match guard.try_io(|stream| {
                stream
                    .get_ref()
                    .send_vectored_with_ancillary(&slices[..cnt], &mut ancillary)
            }) {
                Ok(Ok(len)) => {
                    state.buffer.advance(len);

                    if len == 0 {
                        return Poll::Ready(Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "failed to \
                     write frame to transport",
                        )
                        .into()));
                    }

                    return Poll::Ready(Ok(()));
                }
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(_would_block) => continue,
            }
        }

        Poll::Ready(Ok(()))
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        ready!(self.as_mut().poll_flush(cx))?;
        self.stream.get_ref().shutdown(std::net::Shutdown::Write)?;

        Poll::Ready(Ok(()))
    }
}

impl Socket {
    pub fn new(stream: UnixStream) -> Self {
        Self {
            stream: AsyncFd::new(stream).unwrap(),
            codec: MessageCodec::new(),
            read_state: ReadState::new(),
            write_state: WriteState::new(),
        }
    }

    pub fn poll_read_buf<B: BufMut>(
        stream: &mut AsyncFd<UnixStream>,
        cx: &mut Context<'_>,
        buf: &mut B,
        fds: &mut Vec<RawFd>,
    ) -> Poll<io::Result<usize>> {
        if !buf.has_remaining_mut() {
            return Poll::Ready(Ok(0));
        }

        let n = {
            let dst = buf.chunk_mut();

            // Safety: `chunk_mut()` returns a `&mut UninitSlice`, and `UninitSlice` is a
            // transparent wrapper around `[MaybeUninit<u8>]`.
            let dst = unsafe { &mut *(dst as *mut _ as *mut [MaybeUninit<u8>]) };
            let mut buf = ReadBuf::uninit(dst);
            let ptr = buf.filled().as_ptr();
            ready!(Self::poll_read(stream, cx, &mut buf, fds)?);

            // Ensure the pointer does not change from under us
            assert_eq!(ptr, buf.filled().as_ptr());
            buf.filled().len()
        };

        // Safety: This is guaranteed to be the number of initialized (and read)
        // bytes due to the invariants provided by `ReadBuf::filled`.
        unsafe {
            buf.advance_mut(n);
        }

        Poll::Ready(Ok(n))
    }

    pub fn poll_read(
        stream: &mut AsyncFd<UnixStream>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
        fds: &mut Vec<RawFd>,
    ) -> Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(stream.poll_read_ready(cx))?;

            let mut temp_buf = [0; 128];
            let mut ancillary = SocketAncillary::new(&mut temp_buf);

            let unfilled = buf.initialize_unfilled();

            match guard.try_io(|stream| {
                stream
                    .get_ref()
                    .recv_vectored_with_ancillary(&mut [IoSliceMut::new(unfilled)], &mut ancillary)
            }) {
                Ok(Ok(len)) => {
                    for ancillary_result in ancillary.messages() {
                        if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {
                            for fd in scm_rights {
                                trace!("receive file descriptor: {fd}");
                                fds.push(fd);
                            }
                        }
                    }

                    buf.advance(len);
                    return Poll::Ready(Ok(()));
                }
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(_would_block) => continue,
            }
        }
    }
}

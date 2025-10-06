use futures_sink::Sink;
use pin_project_lite::pin_project;
use tokio::net::UnixStream;
use tokio_stream::Stream;

use waynest::{Message, ObjectId, ProtocolError, Socket};
use waynest_server::Store;

use crate::VerdiError;

pin_project! {
    pub struct Client {
        #[pin]
        socket: Socket,
        store: Store<Client, VerdiError>,
        next_object_id: ObjectId,
        next_event_serial: u32,
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish()
    }
}

impl waynest::Connection for Client {
    type Error = VerdiError;

    fn fd(&mut self) -> std::result::Result<std::os::unix::prelude::OwnedFd, VerdiError> {
        self.socket.fd().map_err(VerdiError::from)
    }
}

impl Stream for Client {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().socket.poll_next(cx)
    }
}

impl Sink<Message> for Client {
    type Error = ProtocolError;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_ready(cx)
    }

    fn start_send(
        self: std::pin::Pin<&mut Self>,
        item: Message,
    ) -> std::result::Result<(), Self::Error> {
        self.project().socket.start_send(item)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_close(cx)
    }
}

impl waynest_server::Client for Client {
    type Store = Store<Client, VerdiError>;

    fn store(&self) -> &Self::Store {
        &self.store
    }

    fn store_mut(&mut self) -> &mut Self::Store {
        &mut self.store
    }
}

impl Client {
    pub fn new(stream: UnixStream) -> Result<Self, VerdiError> {
        Ok(Self {
            socket: Socket::new(stream.into_std()?)?,
            store: Store::new(),
            next_object_id: unsafe { ObjectId::from_raw(0xff000000) },
            next_event_serial: 0,
        })
    }

    pub fn next_event_serial(&mut self) -> u32 {
        let prev = self.next_event_serial;
        self.next_event_serial = self.next_event_serial.wrapping_add(1);

        prev
    }

    // pub fn next_object_id(&mut self) -> usize {
    //     let prev = self.next_object_id;
    //     self.next_object_id = self.next_object_id.wrapping_add(1);

    //     prev
    // }
}

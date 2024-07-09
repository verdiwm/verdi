#![no_main]
use arbitrary::Arbitrary;
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use std::os::fd::RawFd;
use verdi::wire::Message;

#[derive(Debug, Arbitrary)]
struct Data {
    bytes: BytesMut,
    fds: Vec<RawFd>,
}

fuzz_target!(|data: Data| {
    let mut bytes = data.bytes;
    let mut fds = data.fds;

    let _ = Message::from_bytes(&mut bytes, &mut fds);
});

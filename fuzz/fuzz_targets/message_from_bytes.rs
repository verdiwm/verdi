#![no_main]
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use verdi::message::Message;

fuzz_target!(|data: &[u8]| {
    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(data);

    let _ = Message::from_bytes(&mut bytes);
});

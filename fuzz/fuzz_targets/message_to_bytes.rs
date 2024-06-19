#![no_main]
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use verdi::message::Message;

fuzz_target!(|message: Message| {
    let mut bytes = BytesMut::new();

    message.to_bytes(&mut bytes);
});

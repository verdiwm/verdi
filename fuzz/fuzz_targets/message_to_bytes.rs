#![no_main]
use bytes::BytesMut;
use libfuzzer_sys::fuzz_target;
use verdi::wire::Message;

fuzz_target!(|message: Message| {
    let mut bytes = BytesMut::new();
    let mut fds = Vec::new();

    message.to_bytes(&mut bytes, &mut fds);
});

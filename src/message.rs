use bytes::{Buf, Bytes, BytesMut};
use tokio_util::codec::Decoder;

#[derive(Debug)]
pub struct Message {
    pub sender_id: u32,
    pub opcode: u16,
    pub payload: Bytes,
}

impl Message {}

pub struct MessageCodec;

impl MessageCodec {
    pub const fn new() -> Self {
        Self
    }
}

impl Decoder for MessageCodec {
    type Item = Message;

    type Error = anyhow::Error;

    // TODO: error handling
    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> std::result::Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        let sender_id = src.get_u32_ne();
        let second = src.get_u32_ne();
        let len = (second >> 16) as usize;
        let opcode = (second & 65535) as u16;

        let payload = src.copy_to_bytes(len - 8);

        Ok(Some(Message {
            sender_id,
            opcode,
            payload,
        }))
    }
}

use arbitrary::Arbitrary;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Arbitrary)]
pub struct Message {
    pub object_id: u32,
    pub opcode: u16,
    pub payload: Bytes,
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Malformed header")]
    MalformedHeader,
    #[error("Invalid payload lenght")]
    InvalidLenght,
    #[error("Malformed payload")]
    MalformedPayload,
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}

impl Message {
    pub fn to_bytes(&self, buf: &mut BytesMut) {
        buf.reserve(8 + self.payload.len());
        buf.put_u32_ne(self.object_id);
        buf.put_u32_ne((((self.payload.len() + 8) as u32) << 16) | self.opcode as u32);
        buf.put_slice(&self.payload);
    }

    pub fn from_bytes(bytes: &mut BytesMut) -> Result<Self, DecodeError> {
        if bytes.remaining() < 8 {
            return Err(DecodeError::MalformedHeader);
        }

        let object_id = bytes.get_u32_ne();
        let second = bytes.get_u32_ne();
        let len = (second >> 16) as usize;
        let opcode = (second & 65535) as u16;

        if len < 8 {
            return Err(DecodeError::InvalidLenght);
        }

        if bytes.remaining() < (len - 8) {
            return Err(DecodeError::MalformedPayload);
        }

        let payload = bytes.copy_to_bytes(len - 8);

        Ok(Message {
            object_id,
            opcode,
            payload,
        })
    }
}

#[derive(Debug)]
pub struct MessageCodec;

impl MessageCodec {
    pub const fn new() -> Self {
        Self
    }
}

impl Decoder for MessageCodec {
    type Item = Message;

    type Error = DecodeError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> std::result::Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        Message::from_bytes(src).map(Option::Some)
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.to_bytes(dst);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::Message;

    #[test]
    fn encode_decode_roundtrip() {
        let msg = Message {
            object_id: 10,
            opcode: 0,
            payload: Bytes::copy_from_slice(b"\x03\0\0\0"),
        };

        let mut bytes = BytesMut::new();
        msg.to_bytes(&mut bytes);

        assert_eq!(
            msg,
            Message::from_bytes(&mut bytes).expect("Failed to parse bytes")
        );
    }
}

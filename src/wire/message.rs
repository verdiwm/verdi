use arbitrary::Arbitrary;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use super::{DecodeError, Fixed, NewId, ObjectId};

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Arbitrary)]
pub struct Message {
    pub object_id: ObjectId,
    pub opcode: u16,
    pub payload: Bytes,
}

impl Message {
    pub fn new(object_id: ObjectId, opcode: u16, payload: Bytes) -> Self {
        Self {
            object_id,
            opcode,
            payload,
        }
    }

    pub fn to_bytes(&self, buf: &mut BytesMut) {
        buf.reserve(8 + self.payload.len());
        buf.put_u32_ne(self.object_id.as_raw());
        buf.put_u32_ne((((self.payload.len() + 8) as u32) << 16) | self.opcode as u32);
        buf.put_slice(&self.payload);
    }

    pub fn from_bytes(bytes: &mut BytesMut) -> Result<Self, DecodeError> {
        if bytes.remaining() < 8 {
            return Err(DecodeError::MalformedHeader);
        }

        let object_id = bytes.get_u32_ne();

        if object_id == 0 {
            return Err(DecodeError::MalformedHeader);
        }

        let object_id = unsafe { ObjectId::from_raw(object_id) };

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

    pub fn int(&mut self) -> Result<i32, DecodeError> {
        if self.payload.remaining() < 4 {
            return Err(DecodeError::MalformedPayload);
        }

        Ok(self.payload.get_i32_ne())
    }

    pub fn uint(&mut self) -> Result<u32, DecodeError> {
        if self.payload.remaining() < 4 {
            return Err(DecodeError::MalformedPayload);
        }

        Ok(self.payload.get_u32_ne())
    }

    pub fn fixed(&mut self) -> Result<Fixed, DecodeError> {
        self.uint().map(|raw| unsafe { Fixed::from_raw(raw) })
    }

    pub fn string(&mut self) -> Result<Option<String>, DecodeError> {
        let mut array = self.array()?;

        if array.len() == 0 {
            return Ok(None);
        }

        if let Some(b'\0') = array.pop() {
            return String::from_utf8(array)
                .map_err(|_| DecodeError::MalformedPayload)
                .map(Some);
        }

        return Err(DecodeError::MalformedPayload);
    }

    pub fn object(&mut self) -> Result<Option<ObjectId>, DecodeError> {
        if self.payload.remaining() < 4 {
            return Err(DecodeError::MalformedPayload);
        }

        Ok(ObjectId::new(self.payload.get_u32_ne()))
    }

    pub fn new_id(&mut self) -> Result<NewId, DecodeError> {
        let interface = self.string()?.ok_or(DecodeError::MalformedPayload)?;
        let version = self.uint()?;
        let id = self.object()?.ok_or(DecodeError::MalformedPayload)?;

        Ok(NewId {
            interface,
            version,
            id,
        })
    }

    pub fn array(&mut self) -> Result<Vec<u8>, DecodeError> {
        let len = self.uint()? as usize;

        if len == 0 {
            return Ok(Vec::new());
        }

        if self.payload.remaining() < len {
            return Err(DecodeError::MalformedPayload);
        }

        let array = self.payload.copy_to_bytes(len).to_vec();

        self.payload.advance(self.payload.remaining() % 4);

        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use crate::wire::{Message, ObjectId};

    #[test]
    fn encode_decode_roundtrip() {
        let msg = Message {
            object_id: unsafe { ObjectId::from_raw(10) },
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

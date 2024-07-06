use arbitrary::Arbitrary;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::num::NonZeroU32;
use tokio_util::codec::{Decoder, Encoder};

pub struct Fixed(u32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Arbitrary, Clone, Copy)]
#[repr(transparent)]
pub struct ObjectId(NonZeroU32);

impl ObjectId {
    pub const fn as_raw(&self) -> u32 {
        self.0.get()
    }

    pub const unsafe fn from_raw(id: u32) -> Self {
        Self(NonZeroU32::new_unchecked(id))
    }
}

pub struct NewId {
    pub interface: String,
    pub version: u32,
    pub id: ObjectId,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Arbitrary)]
pub struct Message {
    pub object_id: ObjectId,
    pub opcode: u16,
    pub payload: Bytes,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Arbitrary)]
pub struct PayloadBuilder(BytesMut);

impl PayloadBuilder {
    pub fn new() -> Self {
        Self(BytesMut::new())
    }

    pub fn put_int(mut self, int: i32) -> Self {
        self.0.reserve(4);
        self.0.put_i32_ne(int);

        self
    }

    pub fn put_uint(mut self, uint: u32) -> Self {
        self.0.reserve(4);
        self.0.put_u32_ne(uint);

        self
    }

    pub fn put_fixed(mut self, fixed: Fixed) -> Self {
        self.0.reserve(4);
        self.0.put_u32_ne(fixed.0);

        self
    }

    pub fn put_string<T: AsRef<str>>(mut self, string: Option<T>) -> Self {
        if let Some(string) = string {
            let string = string.as_ref();
            let total_len = 5 + string.len();
            let mut padding = 0;

            if total_len % 4 != 0 {
                padding = 4 - (total_len % 4);
            }

            self.0.reserve(total_len + padding);
            self.0.put_u32_ne((string.len() + 1) as u32);
            self.0.put_slice(string.as_bytes());
            self.0.put_u8(b'\0');

            for _ in 0..padding {
                self.0.put_u8(0);
            }

            return self;
        }

        self.put_uint(0)
    }

    pub fn put_object(mut self, object: Option<ObjectId>) -> Self {
        todo!();
        self
    }

    pub fn put_new_id(mut self, new_id: NewId) -> Self {
        todo!();
        self
    }

    pub fn put_array<T: AsRef<[u8]>>(mut self, array: T) -> Self {
        todo!();
        self
    }

    pub fn build(self) -> Bytes {
        self.0.freeze()
    }
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
    pub fn new(object_id: ObjectId, opcode: u16, payload: Bytes) -> Self {
        Self {
            object_id,
            opcode,
            payload,
        }
    }

    pub fn to_bytes(&self, buf: &mut BytesMut) {
        buf.reserve(8 + self.payload.len());
        buf.put_u32_ne(self.object_id.0.get());
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
        self.uint().map(Fixed)
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

        Ok(NonZeroU32::new(self.payload.get_u32_ne()).map(ObjectId))
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

    use crate::message::ObjectId;

    use super::Message;

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

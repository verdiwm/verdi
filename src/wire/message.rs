use std::os::fd::RawFd;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use rustix::fd::{FromRawFd, OwnedFd};

use super::{DecodeError, Fixed, NewId, ObjectId};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
pub struct Message {
    pub object_id: ObjectId,
    pub opcode: u16,
    payload: Bytes,
    fds: Vec<RawFd>,
}

impl Message {
    pub fn new(object_id: ObjectId, opcode: u16, payload: Bytes, fds: Vec<RawFd>) -> Self {
        Self {
            object_id,
            opcode,
            payload,
            fds,
        }
    }

    pub fn to_bytes(&self, buf: &mut BytesMut, fds: &mut Vec<RawFd>) {
        buf.reserve(8 + self.payload.len());
        buf.put_u32_ne(self.object_id.as_raw());
        buf.put_u32_ne((((self.payload.len() + 8) as u32) << 16) | self.opcode as u32);
        buf.put_slice(&self.payload);

        fds.extend_from_slice(&self.fds);
    }

    pub fn from_bytes(bytes: &mut BytesMut, fds: &mut Vec<RawFd>) -> Result<Self, DecodeError> {
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
            fds: fds.clone(),
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
        self.uint().map(ObjectId::new)
    }

    pub fn new_id(&mut self) -> Result<NewId, DecodeError> {
        let interface = self.string()?.ok_or(DecodeError::MalformedPayload)?;
        let version = self.uint()?;
        let object_id = self.object()?.ok_or(DecodeError::MalformedPayload)?;

        Ok(NewId {
            interface,
            version,
            object_id,
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

    pub fn fd(&mut self) -> Result<OwnedFd, DecodeError> {
        self.fds
            .pop()
            .map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })
            .ok_or(DecodeError::MalformedPayload)
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
            fds: vec![10, 20, 0, 33, 48, 17],
        };

        let mut bytes = BytesMut::new();
        let mut fds = Vec::new();
        msg.to_bytes(&mut bytes, &mut fds);

        assert_eq!(
            msg,
            Message::from_bytes(&mut bytes, &mut fds).expect("Failed to parse bytes")
        );
    }
}

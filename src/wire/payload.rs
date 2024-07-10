use std::os::fd::RawFd;

use bytes::{BufMut, Bytes, BytesMut};
use rustix::fd::AsRawFd;

use super::{Fixed, NewId, ObjectId};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
pub struct PayloadBuilder {
    payload: BytesMut,
    fds: Vec<RawFd>,
}

impl PayloadBuilder {
    pub fn new() -> Self {
        Self {
            payload: BytesMut::new(),
            fds: Vec::new(),
        }
    }

    pub fn put_int(mut self, int: i32) -> Self {
        self.payload.reserve(4);
        self.payload.put_i32_ne(int);

        self
    }

    pub fn put_uint(mut self, uint: u32) -> Self {
        self.payload.reserve(4);
        self.payload.put_u32_ne(uint);

        self
    }

    pub fn put_fixed(mut self, fixed: Fixed) -> Self {
        self.payload.reserve(4);
        self.payload.put_u32_ne(fixed.into_raw());

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

            self.payload.reserve(total_len + padding);
            self.payload.put_u32_ne((string.len() + 1) as u32);
            self.payload.put_slice(string.as_bytes());
            self.payload.put_u8(b'\0');

            for _ in 0..padding {
                self.payload.put_u8(0);
            }

            return self;
        }

        self.put_uint(0)
    }

    pub fn put_object(self, object: Option<ObjectId>) -> Self {
        if let Some(id) = object {
            return self.put_uint(id.as_raw());
        }

        self.put_uint(0)
    }

    pub fn put_new_id(self, new_id: NewId) -> Self {
        self.put_string(Some(new_id.interface))
            .put_uint(new_id.version)
            .put_object(Some(new_id.object_id))
    }

    pub fn put_array<T: AsRef<[u8]>>(mut self, array: T) -> Self {
        let array = array.as_ref();
        let total_len = 4 + array.len();
        let mut padding = 0;

        if total_len % 4 != 0 {
            padding = 4 - (total_len % 4);
        }

        self.payload.reserve(total_len + padding);
        self.payload.put_u32_ne(array.len() as u32);
        self.payload.put_slice(array);

        for _ in 0..padding {
            self.payload.put_u8(0);
        }

        self
    }

    pub fn put_fd<Fd: AsRawFd>(mut self, fd: Fd) -> Self {
        self.fds.push(fd.as_raw_fd());

        self
    }

    pub fn build(self) -> (Bytes, Vec<RawFd>) {
        (self.payload.freeze(), self.fds)
    }
}

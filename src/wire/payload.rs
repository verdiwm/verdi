use arbitrary::Arbitrary;
use bytes::{BufMut, Bytes, BytesMut};

use super::{Fixed, NewId, ObjectId};

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
        self.0.put_u32_ne(fixed.into_raw());

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

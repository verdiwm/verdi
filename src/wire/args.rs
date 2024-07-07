use arbitrary::Arbitrary;
use std::num::NonZeroU32;

pub struct Fixed(u32);

impl Fixed {
    pub const unsafe fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn as_raw(&self) -> u32 {
        self.0
    }

    pub const fn into_raw(self) -> u32 {
        self.0
    }
}

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

    pub fn new(id: u32) -> Option<Self> {
        NonZeroU32::new(id).map(Self)
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub struct NewId {
    pub interface: String,
    pub version: u32,
    pub id: ObjectId,
}

use std::{fmt, num::NonZeroU32};

pub struct Fixed(u32);

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[repr(transparent)]
pub struct ObjectId(NonZeroU32);

impl ObjectId {
    pub const DISPLAY: Self = unsafe { Self::from_raw(1) };

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
    pub object_id: ObjectId,
}

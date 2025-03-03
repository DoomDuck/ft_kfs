use core::{fmt, ops::{Deref, DerefMut}, str::FromStr};

use super::ArrayVec;

#[derive(Default, PartialEq, Eq)]
pub struct ArrayStr<const CAPACITY: usize> {
    bytes: ArrayVec<CAPACITY, u8>, 
}

impl<const CAPACITY: usize> ArrayStr<CAPACITY> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), ()> {
        self.bytes.extend_from_slice(s.as_bytes())
    }
}

impl<const CAPACITY: usize> AsRef<str> for ArrayStr<CAPACITY> {
    fn as_ref(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.bytes)
        }
    }
}

impl<const CAPACITY: usize> AsMut<str> for ArrayStr<CAPACITY> {
    fn as_mut(&mut self) -> &mut str {
        unsafe {
            core::str::from_utf8_unchecked_mut(&mut self.bytes)
        }
    }
}

impl<const CAPACITY: usize> FromStr for ArrayStr<CAPACITY> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::new();
        result.push_str(s)?;
        Ok(result)
    }
}

impl<const CAPACITY: usize> Deref for ArrayStr<CAPACITY> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<const CAPACITY: usize> DerefMut for ArrayStr<CAPACITY> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<const CAPACITY: usize> fmt::Write for ArrayStr<CAPACITY> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s).map_err(|_| fmt::Error)
    }
}

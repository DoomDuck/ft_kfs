use core::{ops::{Deref, DerefMut}, str::FromStr};

use super::ArrayVec;

#[derive(Default, PartialEq, Eq)]
pub struct ArrayStr<const CAPACITY: usize> {
    bytes: ArrayVec<CAPACITY, u8>, 
}

impl<const CAPACITY: usize> ArrayStr<CAPACITY> {
    pub fn new() -> Self {
        Self::default()
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
        let mut bytes = ArrayVec::new();
        match bytes.get_mut(..s.len()) {
            None => return Err(()),
            Some(slice) => slice.copy_from_slice(s.as_bytes()),
        }
        Ok(ArrayStr { bytes })
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

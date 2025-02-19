use core::str::FromStr;

pub struct ArrayStr<const CAPACITY: usize> {
    len: usize,
    values: [u8; CAPACITY],
}

impl<const CAPACITY: usize> FromStr for ArrayStr<CAPACITY> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = ArrayStr {
            len: s.len(),
            values: [0u8; CAPACITY],
        };
        match result.values.get_mut(..s.len()) {
            None => return Err(()),
            Some(slice) => slice.copy_from_slice(s.as_bytes()),
        }
        Ok(result)
    }
}

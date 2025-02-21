use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

pub struct ArrayVec<const CAPACITY: usize, T> {
    len: usize,
    values: [MaybeUninit<T>; CAPACITY],
}

impl<const CAPACITY: usize, T> ArrayVec<CAPACITY, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub unsafe fn push_unchecked(&mut self, value: T) {
        self.values.get_unchecked_mut(self.len).write(value);
        self.len += 1;
    }

    // Maybe return a reference to the inserted value
    pub fn push(&mut self, value: T) -> Result<(), T> {
        match self.len < CAPACITY {
            false => Err(value),
            true => unsafe {
                self.push_unchecked(value);
                Ok(())
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match 0 < self.len {
            false => None,
            true => unsafe {
                self.len -= 1;
                Some(self.values.get_unchecked_mut(self.len).assume_init_read())
            },
        }
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        match self.len < CAPACITY && index <= self.len {
            false => Err(value),
            true => unsafe {
                // Leave room
                let start = &mut self.values[index] as *mut MaybeUninit<T>;
                core::ptr::copy(start, start.add(1), self.len - index);

                // Insert value
                self.values.get_unchecked_mut(index).write(value);
                self.len += 1;
                Ok(())
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_ref().iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.as_mut().iter_mut()
    }

}

impl<const CAPACITY: usize, T: Clone> ArrayVec<CAPACITY, T> {
    pub fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), ()> {
        match self.len() + slice.len() <= CAPACITY {
            false => Err(()),
            true => unsafe {
                for value in slice {
                    self.push_unchecked(value.clone());
                }
                Ok(())
            },
        }
    }
}

impl<const CAPACITY: usize, T> Drop for ArrayVec<CAPACITY, T> {
    fn drop(&mut self) {
        // Drain the structure
        while let Some(_value) = self.pop() {}
    }
}

impl<const CAPACITY: usize, T> AsRef<[T]> for ArrayVec<CAPACITY, T> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            let slice = self.values.as_slice().get_unchecked(..self.len);
            &*(slice as *const [MaybeUninit<T>] as *const [T])
        }
    }
}

impl<const CAPACITY: usize, T> AsMut<[T]> for ArrayVec<CAPACITY, T> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe {
            let slice = self.values.as_mut_slice().get_unchecked_mut(..self.len);
            &mut *(slice as *mut [MaybeUninit<T>] as *mut [T])
        }
    }
}

impl<const CAPACITY: usize, T> Deref for ArrayVec<CAPACITY, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<const CAPACITY: usize, T> DerefMut for ArrayVec<CAPACITY, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<const CAPACITY: usize, T> Default for ArrayVec<CAPACITY, T> {
    fn default() -> Self {
        Self {
            len: 0,
            values: [const { MaybeUninit::uninit() }; CAPACITY],
        }
    }
}

impl<const CAPACITY: usize, T: PartialEq> PartialEq for ArrayVec<CAPACITY, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<const CAPACITY: usize, T: Eq> Eq for ArrayVec<CAPACITY, T> {}

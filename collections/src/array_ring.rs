use core::mem::MaybeUninit;

pub struct ArrayRing<const CAPACITY: usize, T> {
    values: [MaybeUninit<T>; CAPACITY],
    len: usize,
    start: usize,
}

impl<const CAPACITY: usize, T> ArrayRing<CAPACITY, T> {
    pub const fn new() -> Self {
        Self {
            values: [const { MaybeUninit::uninit() }; CAPACITY],
            len: 0,
            start: 0,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == CAPACITY
    }

    unsafe fn get_unchecked(&self, index: usize) -> &MaybeUninit<T> {
        self.values.get_unchecked((self.start + index) % CAPACITY)
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut MaybeUninit<T> {
        self.values.get_unchecked_mut((self.start + index) % CAPACITY)
    }

    // Maybe return a reference to the inserted value
    pub fn push_front(&mut self, value: T) -> Result<(), T> {
        match self.len < CAPACITY {
            false => Err(value),
            true => unsafe {
                self.start = (self.start + CAPACITY - 1) % CAPACITY;
                self.get_unchecked_mut(0).write(value);
                self.len += 1;
                Ok(())
            },
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match 0 < self.len {
            false => None,
            true => unsafe {
                let value = self.get_unchecked_mut(0).assume_init_read();
                self.start = (self.start + 1) % self.len;
                self.len -= 1;
                Some(value)
            },
        }
    }

    // Maybe return a reference to the inserted value
    pub fn push_back(&mut self, value: T) -> Result<(), T> {
        match self.len < CAPACITY {
            false => Err(value),
            true => unsafe {
                self.get_unchecked_mut(self.len).write(value);
                self.len += 1;
                Ok(())
            },
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match 0 < self.len {
            false => None,
            true => unsafe {
                self.len -= 1;
                Some(self.get_unchecked_mut(self.len).assume_init_read())
            },
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match index < self.len {
            false => None,
            true => unsafe {
                Some(self.get_unchecked(index).assume_init_ref())
            }
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.len {
            false => None,
            true => unsafe {
                Some(self.get_unchecked_mut(index).assume_init_mut())
            }
        }
    }

    // pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
    //     match self.len < CAPACITY && index <= self.len {
    //         false => Err(value),
    //         true => unsafe {
    //             // Leave room
    //             let start = &mut self.values[index] as *mut MaybeUninit<T>;
    //             core::ptr::copy(start, start.add(1), self.len - index);
    //
    //             // Insert value
    //             self.values.get_unchecked_mut(index).write(value);
    //             self.len += 1;
    //             Ok(())
    //         },
    //     }
    // }

    pub fn as_slices(&self) -> (&[T], &[T]) {
        unsafe {
            match self.start + self.len <= CAPACITY {
                false => {
                    let start_slice = self.values.as_slice().get_unchecked(self.start..);
                    let start_slice = &*(start_slice as *const [MaybeUninit<T>] as *const [T]);
                    let end_slice = self.values.as_slice().get_unchecked(self.start..);
                    let end_slice = &*(end_slice as *const [MaybeUninit<T>] as *const [T]);
                    (start_slice, end_slice)
                }
                true => {
                    let end = self.start + self.len;
                    let slice = self.values.as_slice().get_unchecked(self.start..end);
                    let slice = &*(slice as *const [MaybeUninit<T>] as *const [T]);
                    (slice, &[])
                }
            }
        }
    }

    pub fn as_slices_mut(&mut self) -> (&mut [T], &mut [T]) {
        unsafe {
            match self.start + self.len <= CAPACITY {
                false => {
                    let start_slice = self.values.as_mut_slice().get_unchecked_mut(self.start..);
                    let start_slice = &mut *(start_slice as *mut [MaybeUninit<T>] as *mut [T]);
                    let end_slice = self.values.as_mut_slice().get_unchecked_mut(self.start..);
                    let end_slice = &mut *(end_slice as *mut [MaybeUninit<T>] as *mut [T]);
                    (start_slice, end_slice)
                }
                true => {
                    let end = self.start + self.len;
                    let slice = self.values.as_mut_slice().get_unchecked_mut(self.start..end);
                    let slice = &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]);
                    (slice, &mut [])
                }
            }
        }
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        let (start, end) = self.as_slices();
        start.iter().chain(end)
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        let (start, end) = self.as_slices_mut();
        start.iter_mut().chain(end)
    }
}

impl<const CAPACITY: usize, T> Default for ArrayRing<CAPACITY, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize, T> Drop for ArrayRing<CAPACITY, T> {
    fn drop(&mut self) {
        // Drain the structure
        while let Some(_value) = self.pop_back() {}
    }
}

impl<const CAPACITY: usize, T> core::ops::Index<usize> for ArrayRing<CAPACITY, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of range")
    }
}

impl<const CAPACITY: usize, T> core::ops::IndexMut<usize> for ArrayRing<CAPACITY, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of range")
    }
}

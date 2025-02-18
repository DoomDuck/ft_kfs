#![no_std]

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T: ?Sized> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Sync> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T: ?Sized> SpinLock<T> {
    pub const fn new(data: T) -> Self
    where
        T: Sized,
    {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        SpinLockGuard(&self)
    }

    unsafe fn unlock_unchecked(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

pub struct SpinLockGuard<'a, T: ?Sized>(&'a SpinLock<T>);

impl<'a, T: ?Sized> AsRef<T> for SpinLockGuard<'a, T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.0.data.get() as _ }
    }
}

impl<'a, T: ?Sized> AsMut<T> for SpinLockGuard<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0.data.get() as _ }
    }
}

impl<'a, T: ?Sized> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: ?Sized> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<'a, T: ?Sized> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.0.unlock_unchecked();
        }
    }
}

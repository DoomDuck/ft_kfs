#![no_std]

pub use asm::IOPort as ID;
use sync::SpinLock;

pub static MANAGER: SpinLock<Manager> = SpinLock::new(Manager::new());

pub struct Manager {
    used: [bool; Self::PORT_COUNT],
}

impl Manager {
    const PORT_COUNT: usize = ID::MAX as usize + 1;

    const fn new() -> Self {
        Self {
            used: [false; ID::MAX as usize + 1],
        }
    }

    pub fn try_aquire<const PORT: ID>(&mut self) -> Result<Handle<PORT>, ()> {
        match self.used[PORT as usize] {
            true => Err(()),
            ref mut is_used => {
                *is_used = false;
                Ok(Handle::new())
            }
        }
    }
}

pub struct Handle<const PORT: ID> {
    _prevent_construction_outside_crate: (),
}

impl<const PORT: ID> Handle<PORT> {
    const fn new() -> Self {
        Self {
            _prevent_construction_outside_crate: (),
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        asm::in8(PORT)
    }

    pub fn read_u16(&mut self) -> u16 {
        asm::in16(PORT)
    }

    pub fn read_u32(&mut self) -> u32 {
        asm::in32(PORT)
    }

    pub fn write_u8(&mut self, value: u8) {
        asm::out8(PORT, value);
    }

    pub fn write_u16(&mut self, value: u16) {
        asm::out16(PORT, value)
    }

    pub fn write_u32(&mut self, value: u32) {
        asm::out32(PORT, value)
    }
}

impl<const PORT: ID> Drop for Handle<PORT> {
    fn drop(&mut self) {
        MANAGER.lock().used[PORT as usize] = false;
    }
}

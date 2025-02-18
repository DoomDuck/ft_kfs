#![no_std]

pub type IOPort = u16;

pub fn in8(port: IOPort) -> u8 {
    let mut result;
    unsafe {
        // TODO: Check in instruction arg order
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") result,
        );
    }
    result
}

pub fn out8(port: IOPort, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
        );
    }
}

pub fn in16(port: IOPort) -> u16 {
    let mut result;
    unsafe {
        core::arch::asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") result,
        );
    }
    result
}

pub fn out16(port: IOPort, value: u16) {
    unsafe {
        core::arch::asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") value,
        );
    }
}

pub fn in32(port: IOPort) -> u32 {
    let mut result;
    unsafe {
        core::arch::asm!(
            "in eax, dx",
            in("dx") port,
            out("eax") result,
        );
    }
    result
}

pub fn out32(port: IOPort, value: u32) {
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
        );
    }
}

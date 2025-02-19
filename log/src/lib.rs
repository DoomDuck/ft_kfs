#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};

const START_LINE: usize = 0;
const START_COLUMN: usize = 0;
static LINE: AtomicUsize = AtomicUsize::new(START_LINE);
static COLUMN: AtomicUsize = AtomicUsize::new(START_COLUMN);

pub struct Logger;

impl Logger {
    fn write_char(&self, buffer: &mut vga::TextBuffer, to_write: u8) {
        if to_write == b'\n' {
            COLUMN.store(0, SeqCst);
            LINE.fetch_add(1, SeqCst);
        } else {
            buffer[LINE.load(SeqCst)][COLUMN.load(SeqCst)] = vga::Char::new(to_write);
            COLUMN.fetch_add(1, SeqCst);
            if COLUMN.load(SeqCst) >= vga::TextBuffer::WIDTH {
                COLUMN.store(0, SeqCst);
                LINE.fetch_add(1, SeqCst);
            }
        }
        if LINE.load(SeqCst) >= vga::TextBuffer::HEIGHT {
            LINE.store(START_LINE, SeqCst);
        }
    }

    pub fn log(&self, buffer: &mut vga::TextBuffer, s: &[u8]) {
        for &byte in s {
            self.write_char(buffer, byte);
        }
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let text_buffer = unsafe { vga::TextBuffer::get_mut() };
        self.log(text_buffer, s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    ($fmt:literal $(,$args:expr)*) => {
        {
            use core::fmt::Write;
            let _ = write!(&mut $crate::Logger, $fmt, $($args),*);
        }
    };
}

#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use collections::{ArrayRing, ArrayStr};
use sync::SpinLock;

pub use collections;

// TODO: decide on variants and number order
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub struct Entry {
    pub level: Level,
    pub content: ArrayStr<{Self::MAX_CONTENT_LENGTH}>,
}

impl Entry {
    pub const MAX_CONTENT_LENGTH: usize = 0x100;
}

#[derive(Default)]
pub struct Logger {
    entries: ArrayRing<{Self::MAX_ENTRY_COUNT}, Entry>,
}

pub static INSTANCE : SpinLock<Logger> = SpinLock::new(Logger::new());

impl Logger {
    pub const MAX_ENTRY_COUNT: usize = 0x100;

    pub const fn new() -> Self {
        Self { entries: ArrayRing::new() }
    }

    pub fn register(&mut self, entry: Entry) {
        // TODO: Maybe create a push function that does this automatical
        if self.entries.is_full() {
            // TODO: replace by an unwrap
            let _ = self.entries.pop_back();
        }
        // TODO: replace by an unwrap
        let _ = self.entries.push_back(entry);
    }

    pub fn entries(&self) -> impl DoubleEndedIterator<Item = &Entry> {
        self.entries.iter()
    }
}

const START_LINE: usize = 0;
const START_COLUMN: usize = 0;
static LINE: AtomicUsize = AtomicUsize::new(START_LINE);
static COLUMN: AtomicUsize = AtomicUsize::new(START_COLUMN);

pub struct VgaLogger;

impl VgaLogger {
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

impl core::fmt::Write for VgaLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let text_buffer = unsafe { vga::TextBuffer::get_mut() };
        self.log(text_buffer, s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $fmt:literal $(,$args:expr)*) => {
        {
            use core::fmt::Write;
            let mut content = $crate::collections::ArrayStr::new();
            let _ = write!(&mut content, $fmt, $($args),*);
            let entry = $crate::Entry { level: $level, content };
            $crate::INSTANCE.lock().register(entry);
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt:literal $(,$args:expr)*) => {
        $crate::log!($crate::Level::Trace, $fmt $(,($args))*)
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:literal $(,$args:expr)*) => {
        $crate::log!($crate::Level::Debug, $fmt $(,($args))*)
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:literal $(,$args:expr)*) => {
        $crate::log!($crate::Level::Info, $fmt $(,($args))*)
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal $(,$args:expr)*) => {
        $crate::log!($crate::Level::Warn, $fmt $(,($args))*)
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:literal $(,$args:expr)*) => {
        $crate::log!($crate::Level::Error, $fmt $(,($args))*)
    };
}

#[macro_export]
macro_rules! vga_log {
    ($fmt:literal $(,$args:expr)*) => {
        {
            use core::fmt::Write;
            let _ = write!(&mut $crate::VgaLogger, $fmt, $($args),*);
        }
    };
}

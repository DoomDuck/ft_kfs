#![no_std]
#![no_main]

mod vga {

    #[repr(transparent)]
    pub struct Color(u8);

    impl Default for Color {
        fn default() -> Self {
            Self::new(Self::WHITE, Self::BLACK)
        }
    }

    impl Color {
        pub const BLACK : u8 = 0x0;
        pub const BLUE: u8 = 0x1;
        pub const GREEN: u8 = 0x2;
        pub const CYAN: u8 = 0x3;
        pub const RED: u8 = 0x4;
        pub const MAGENTA: u8 = 0x5;
        pub const BROWN: u8 = 0x6;
        pub const LIGHT_GRAY: u8 = 0x7;
        pub const DARK_GRAY: u8 = 0x8;
        pub const LIGHT_BLUE: u8 = 0x9;
        pub const LIGHT_GREEN: u8 = 0xa;
        pub const LIGHT_CYAN: u8 = 0xb;
        pub const LIGHT_RED: u8 = 0xc;
        pub const LIGHT_MAGENTA: u8 = 0xd;
        pub const YELLOW: u8 = 0xe;
        pub const WHITE: u8 = 0xf;

        fn new(front: u8, back: u8) -> Color {
            Self(back << 4 | front)
        }
    }

    #[repr(C)]
    pub struct Char {
        code_point: u8, 
        color: Color,
    }

    impl Char {
        pub fn new(code_point: u8) -> Self {
            Self {
                color: Color::default(),
                code_point,
            }
        }
    }

    #[repr(C)]
    pub struct TextBuffer([[Char; Self::WIDTH]; Self::HEIGHT]);

    impl TextBuffer {
        // TODO(Dorian): add reference (see discord)
        pub const WIDTH: usize = 80;
        pub const HEIGHT: usize = 25;
        const LOCATION: *mut TextBuffer  = 0xB8000 as *mut TextBuffer;

        pub unsafe fn get() -> &'static Self {
            &mut *TextBuffer::LOCATION as _
        }

        pub unsafe fn get_mut() -> &'static mut Self {
            &mut *TextBuffer::LOCATION as _
        }
    }


    impl core::ops::Deref for TextBuffer {
        type Target = [[Char; Self::WIDTH]; Self::HEIGHT];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl core::ops::DerefMut for TextBuffer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}


#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
extern "C" fn entrypoint() {
    let text_buffer = unsafe { vga::TextBuffer::get_mut() };
    text_buffer[0][0] = vga::Char::new(b'4');
    text_buffer[0][1] = vga::Char::new(b'2');
}

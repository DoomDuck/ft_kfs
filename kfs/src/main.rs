#![no_std]
#![no_main]

use tui::{TextBuffer, Widget};

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let _ = match info.location() {
        Some(location) => log::vga_log!("panic({location}): {}\n", info.message()),
        None => log::vga_log!("panic: {}\n", info.message()),
    };

    loop {
        core::hint::spin_loop();
    }
}

enum Entry {
    Log(tui::Logger),
    Text(TextBuffer),
}

impl Widget for Entry {
    type Event = keyboard::Event;

    fn render(&self, screen: &mut tui::Screen, area: tui::Rectangle) {
        match self {
            Entry::Log(logger) => logger.render(screen, area),
            Entry::Text(text_buffer) => text_buffer.render(screen, area),
        }
    }

    fn update(&mut self, event: Self::Event) {
        match self {
            Entry::Log(logger) => logger.update(event),
            Entry::Text(text_buffer) => text_buffer.update(event),
        }
    }
}

#[no_mangle]
extern "C" fn entrypoint() {
    log::info!("42");

    let mut port_manager = port::MANAGER.lock();
    let Ok(data_port) = port_manager.try_aquire() else {
        return;
    };
    let Ok(controller_port) = port_manager.try_aquire() else {
        return;
    };

    let mut ps2_controller = ps2::Controller {
        data_port,
        controller_port,
    };

    match ps2_controller.initialize() {
        Ok((port_1_type, port_2_type)) => log::info!("{port_1_type:?}, {port_2_type:?}"),
        Err(()) => log::error!("Could not initialize ps2 ports"),
    }

    let mut decoder = ps2::keyboard::Decoder::ReadNothing;

    let keyboard = keyboard::Keyboard::qwerty();
    // let keyboard = keyboard::Keyboard::ergol();

    let mut screen = tui::Screen::default();

    let mut root_widget = tui::MultiScreen::new([
        Entry::Log(tui::Logger),
        Entry::Text(tui::TextBuffer::new(keyboard)),
    ]);

    loop {
        screen.clear();
        let area = screen.area();
        root_widget.render(&mut screen, area);

        // HACK: here TextBuffer requires to be locked
        unsafe {
            screen.write_to_vga();
        }

        let event = match decoder.feed(ps2_controller.read_without_origin()) {
            Ok(Some(event)) => event,
            Ok(None) => continue,
            Err(err) => panic!("Could not decode ps2 bytes: {err:?}"),
        };

        log::debug!("Got event: {event:?}");
        Widget::update(&mut root_widget, event);
    }
}

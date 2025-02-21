#![no_std]
#![no_main]

use tui::Widget;

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

    // let mut keyboard = keyboard::Keyboard::qwerty();
    let mut keyboard = keyboard::Keyboard::ergol();
    

    let mut screen = tui::Screen::default();

    loop {
        let area = screen.area();
        tui::Logger.render(&mut screen, area);
        
        // HACK: here TextBuffer requires to be locked
        unsafe {
            screen.write_to_vga();
        }

        let event = match decoder.feed(ps2_controller.read_without_origin()) {
            Ok(Some(event)) => event,
            Ok(None) => continue,
            Err(err) => panic!("Could not decode ps2 bytes: {err:?}"),
        };

        log::debug!("Got event: {event:?}")
    }
}

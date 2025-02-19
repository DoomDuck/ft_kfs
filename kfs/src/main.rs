#![no_std]
#![no_main]

use log::log;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let _ = match info.location() {
        Some(location) => log!("panic({location}): {}\n", info.message()),
        None => log!("panic(location): {}\n", info.message()),
    };

    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
extern "C" fn entrypoint() {
    log!("42\n");

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
        Ok((port_1_type, port_2_type)) => log!("{port_1_type:?}, {port_2_type:?}\n"),
        Err(()) => log!("Could not initialize ps2 ports\n"),
    }

    let mut decoder = ps2::keyboard::Decoder::ReadNothing;

    // let mut keyboard = keyboard::Keyboard::qwerty();
    let mut keyboard = keyboard::Keyboard::ergol();

    loop {
        let event = match decoder.feed(ps2_controller.read_without_origin()) {
            Ok(Some(event)) => event,
            Ok(None) => continue,
            Err(err) => panic!("Could not decode ps2 bytes: {err:?}"),
        };

        if let Some(text) = keyboard.feed(event) {
            log::log!("{text}");
        };
    }
}

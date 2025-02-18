#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// global allocator
extern crate alloc;
extern crate flipperzero_alloc;

mod gui;

use alloc::boxed::Box;
use core::ffi::CStr;

use flipperzero::{
    furi::{message_queue::MessageQueue, time::Duration},
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

use gui::{Gui, ViewPort};

manifest!(
    name = "YT Bluetooth Remote",
    app_version = 1,
    has_icon = true,
    icon = "icon.icon",
);

struct State {
    event_queue: MessageQueue<sys::InputEvent>,
}

entry!(main);
fn main(_args: Option<&CStr>) -> i32 {
    println!("Hello, Rust!");

    let state = Box::new(State {
        event_queue: MessageQueue::new(8),
    });

    let mut view_port = ViewPort::new();
    view_port.set_draw_callback(|canvas| {
        canvas.set_font(sys::FontPrimary);
        canvas.draw_str_aligned(1, 1, sys::AlignLeft, sys::AlignTop, c"meow");
    });
    view_port.set_input_callback(|input| {
        state
            .event_queue
            .put(*input, Duration::WAIT_FOREVER)
            .unwrap();
    });

    let mut gui = Gui::new();
    gui.add_view_port(&view_port, sys::GuiLayerFullscreen);

    loop {
        let event = state.event_queue.get(Duration::WAIT_FOREVER).unwrap();
        if event.key == sys::InputKeyBack {
            break;
        }

        view_port.update();
    }

    view_port.set_enabled(false);

    0
}

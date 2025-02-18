#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// global allocator
extern crate alloc;
extern crate flipperzero_alloc;

mod gui;

use core::ffi::CStr;

use flipperzero::{
    furi::{message_queue::MessageQueue, time::Duration},
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

use gui::{Gui, Orientation, ViewPort};

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

    let state = State {
        event_queue: MessageQueue::new(8),
    };

    let mut view_port = ViewPort::new();
    view_port.set_orientation(Orientation::VerticalFlip);
    view_port.set_draw_callback(|canvas| {
        canvas.set_font(sys::FontPrimary);
        canvas.draw_str_aligned(0, 0, sys::AlignLeft, sys::AlignTop, c"meow");
        canvas.set_font(sys::FontSecondary);
        canvas.draw_str_aligned(0, 9, sys::AlignLeft, sys::AlignTop, c"foof");
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

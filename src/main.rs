#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// global allocator
extern crate alloc;
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::{
    furi::{message_queue::MessageQueue, time::Duration},
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

use flipper_yt_remote::gui::{
    view_port::ViewPort, Font, Gui, InputEvent, InputKey, InputType,
    Orientation,
};

manifest!(
    name = "YT Bluetooth Remote",
    app_version = 1,
    has_icon = true,
    icon = concat!(env!("OUT_DIR"), "/icon.icon"),
);

struct State {
    event_queue: MessageQueue<InputEvent>,
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
        canvas.draw_rounded_box(0, 0, 20, 20, 2);
        canvas.draw_rounded_frame(0, 30, 20, 20, 2);
        for (i, f) in [
            Font::Primary,
            Font::Secondary,
            Font::Keyboard,
            Font::BigNumbers,
        ]
        .into_iter()
        .enumerate()
        {
            canvas.set_font(f);
            canvas.draw_str(0, 30 * i as i32 + 10, c"0123");
        }
    });
    view_port.set_input_callback(|input| {
        state
            .event_queue
            .put(input, Duration::WAIT_FOREVER)
            .unwrap();
    });

    let mut gui = Gui::open();
    gui.add_view_port(&view_port, sys::GuiLayerFullscreen);

    loop {
        let event = state.event_queue.get(Duration::WAIT_FOREVER).unwrap();
        if event.type_ == InputType::Long && event.key == InputKey::Back {
            break;
        }

        view_port.update();
    }

    view_port.set_enabled(false);

    0
}

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// global allocator
extern crate alloc;
extern crate flipperzero_alloc;

use core::ffi::{c_void, CStr};

use alloc::boxed::Box;
use flipperzero::{
    furi::{message_queue::MessageQueue, time::Duration},
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys::{
    canvas_draw_str_aligned, canvas_set_font, furi_record_close,
    furi_record_open, gui_add_view_port, gui_remove_view_port, view_port_alloc,
    view_port_draw_callback_set, view_port_enabled_set, view_port_free,
    view_port_input_callback_set, view_port_update, AlignLeft, AlignTop,
    Canvas, FontPrimary, Gui, GuiLayerFullscreen, InputEvent, InputKeyBack,
};

manifest!(
    name = "YT Bluetooth Remote",
    app_version = 1,
    has_icon = true,
    icon = "icon.icon",
);

const RECORD_GUI: &CStr = c"gui";

unsafe extern "C" fn gui_render(canvas: *mut Canvas, _state: *mut c_void) {
    canvas_set_font(canvas, FontPrimary);
    let str = c"foo";
    canvas_draw_str_aligned(canvas, 1, 1, AlignLeft, AlignTop, str.as_ptr());
}
unsafe extern "C" fn gui_input(event: *mut InputEvent, state: *mut c_void) {
    let state = state.cast_const().cast::<State>().as_ref().unwrap();
    let event = unsafe { event.read() };
    state
        .event_queue
        .put(event, Duration::WAIT_FOREVER)
        .unwrap();
}

struct State {
    event_queue: MessageQueue<InputEvent>,
}

entry!(main);
fn main(_args: Option<&CStr>) -> i32 {
    println!("Hello, Rust!");

    let state = Box::new(State {
        event_queue: MessageQueue::new(8),
    });

    let view_port = unsafe { view_port_alloc() };
    unsafe {
        view_port_draw_callback_set(
            view_port,
            Some(gui_render),
            (&raw const *state).cast_mut().cast(),
        );
        view_port_input_callback_set(
            view_port,
            Some(gui_input),
            (&raw const *state).cast_mut().cast(),
        );
    }

    let gui: *mut Gui = unsafe { furi_record_open(RECORD_GUI.as_ptr()) }.cast();
    unsafe { gui_add_view_port(gui, view_port, GuiLayerFullscreen) };

    loop {
        let event = state.event_queue.get(Duration::WAIT_FOREVER).unwrap();
        if event.key == InputKeyBack {
            break;
        }

        unsafe { view_port_update(view_port) };
    }

    unsafe { view_port_enabled_set(view_port, false) };
    unsafe { gui_remove_view_port(gui, view_port) };
    unsafe { view_port_free(view_port) };
    unsafe { furi_record_close(RECORD_GUI.as_ptr()) };

    0
}

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// global allocator
extern crate alloc;
extern crate flipperzero_alloc;

use core::{
    ffi::CStr,
    sync::atomic::{AtomicBool, Ordering},
};

use flipperzero::{
    furi::{message_queue::MessageQueue, time::Duration},
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

use flipper_yt_remote::{
    bt::{Bt, BtStatus, Key, KeyMods},
    gui::{
        view_port::ViewPort, Font, Gui, InputEvent, InputKey, InputType, Orientation
    },
};

manifest!(
    name = "YT Bluetooth Remote",
    app_version = 1,
    has_icon = true,
    icon = concat!(env!("OUT_DIR"), "/icon.icon"),
);

struct State {
    event_queue: MessageQueue<InputEvent>,
    bt_connected: AtomicBool,
}

entry!(main);
fn main(_args: Option<&CStr>) -> i32 {
    println!("Hello, Rust!\r");

    let state = State {
        event_queue: MessageQueue::new(8),
        bt_connected: AtomicBool::new(false),
    };

    let mut bt = Bt::open();
    bt.disconnect();
    unsafe { sys::furi_delay_ms(200) };
    bt.set_key_storage_path(c"/data/.bt_hid.keys");
    bt.set_status_changed_callback(|status| {
        state
            .bt_connected
            .store(status == BtStatus::Connected, Ordering::Relaxed)
    });
    let bt_hid_profile = bt
        .hid_profile_start(sys::BleProfileHidParams {
            device_name_prefix: c"YtRemote".as_ptr(),
            mac_xor: 1,
        })
        .unwrap();
    Bt::start_advertising();

    let mut view_port = ViewPort::new();
    view_port.set_orientation(Orientation::VerticalFlip);
    view_port.set_draw_callback(|canvas| {
        // canvas.draw_rounded_box(0, 0, 20, 20, 2);
        // canvas.draw_rounded_frame(0, 30, 20, 20, 2);
        // for (i, f) in [
        //     Font::Primary,
        //     Font::Secondary,
        //     Font::Keyboard,
        //     Font::BigNumbers,
        // ]
        // .into_iter()
        // .enumerate()
        // {
        //     canvas.set_font(f);
        //     canvas.draw_str(0, 30 * i as i32 + 10, c"0123");
        // }
        let text = if state.bt_connected.load(Ordering::Relaxed) {
            c"on"
        } else {
            c"off"
        };
        canvas.set_font(Font::Primary);
        canvas.draw_str(10, 10, text);
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
        match (event.key, event.type_) {
            (InputKey::Back, InputType::Short) => bt.forget_bonded_devices(),
            (InputKey::Back, InputType::Long) => break,
            (InputKey::Ok, InputType::Short) => {
                let _ = bt_hid_profile.key_press(Key::Spacebar);
                let _ = bt_hid_profile.key_release(Key::Spacebar);
            }
            (InputKey::Ok, InputType::Long) => {
                let _ = bt_hid_profile.key_press(Key::F);
                let _ = bt_hid_profile.key_release(Key::F);
            }
            (InputKey::Left, InputType::Short) => {
                let _ = bt_hid_profile.key_press(Key::LeftArrow);
                let _ = bt_hid_profile.key_release(Key::LeftArrow);
            }
            (InputKey::Right, InputType::Short) => {
                let _ = bt_hid_profile.key_press(Key::RightArrow);
                let _ = bt_hid_profile.key_release(Key::RightArrow);
            }
            (InputKey::Left, InputType::Long) => {
                let _ = bt_hid_profile.key_press(Key::Comma | KeyMods::LeftShift);
                let _ = bt_hid_profile.key_release(Key::Comma | KeyMods::LeftShift);
            }
            (InputKey::Right, InputType::Long) => {
                let _ = bt_hid_profile.key_press(Key::Dot | KeyMods::LeftShift);
                let _ = bt_hid_profile.key_release(Key::Dot | KeyMods::LeftShift);
            }
            (InputKey::Up, InputType::Short) => {
                let _ = bt_hid_profile.key_press(Key::Dot);
                let _ = bt_hid_profile.key_release(Key::Dot);
            }
            (InputKey::Down, InputType::Short) => {
                let _ = bt_hid_profile.key_press(Key::Comma);
                let _ = bt_hid_profile.key_release(Key::Comma);
            }
            (InputKey::Up, InputType::Long) => {
                let _ = bt_hid_profile.consumer_key_press(0xE9);
                let _ = bt_hid_profile.consumer_key_release(0xE9);
            }
            (InputKey::Down, InputType::Long) => {
                let _ = bt_hid_profile.consumer_key_press(0xEA);
                let _ = bt_hid_profile.consumer_key_release(0xEA);
            }
            _ => (),
        }

        view_port.update();
    }

    view_port.set_enabled(false);

    bt.unset_status_changed_callback();
    bt.disconnect();
    unsafe { sys::furi_delay_ms(200) };
    bt.set_default_key_storage_path();
    bt_hid_profile.restore_default_profile().unwrap();

    0
}

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

// global allocator
extern crate alloc;
extern crate flipperzero_alloc;

use core::{
    ffi::CStr,
    sync::atomic::{AtomicBool, AtomicU8, Ordering},
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
        Font, Gui, InputEvent, InputKey, InputType, Orientation,
        view_port::ViewPort,
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
    mode: AtomicU8,
}

entry!(main);
fn main(_args: Option<&CStr>) -> i32 {
    println!("Hello, Rust!\r");

    let state = State {
        event_queue: MessageQueue::new(8),
        bt_connected: AtomicBool::new(false),
        mode: AtomicU8::new(0),
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
        let bt_connected = state.bt_connected.load(Ordering::Relaxed);
        let mode = match state.mode.load(Ordering::Relaxed) {
            0 => Mode::Basic,
            1 => Mode::Mouse,
            _ => unreachable!(),
        };
        let text = if bt_connected {
            c"bluetooth: on"
        } else {
            c"bluetooth: off"
        };
        canvas.set_font(Font::Secondary);
        canvas.draw_str_aligned(0, 0, sys::AlignLeft, sys::AlignTop, text);

        let text = match mode {
            Mode::Basic => c"basic mode",
            Mode::Mouse => c"mouse mode",
        };
        canvas.draw_str_aligned(0, 10, sys::AlignLeft, sys::AlignTop, text);
    });
    view_port.set_input_callback(|input| {
        state
            .event_queue
            .put(input, Duration::WAIT_FOREVER)
            .unwrap();
    });

    let mut gui = Gui::open();
    gui.add_view_port(&view_port, sys::GuiLayerFullscreen);

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Basic = 0,
        Mouse = 1,
    }

    let mut mode = Mode::Basic;

    loop {
        state.mode.store(mode as u8, Ordering::Relaxed);
        let connected = state.bt_connected.load(Ordering::Relaxed);
        let event = state.event_queue.get(Duration::WAIT_FOREVER).unwrap();
        if let Mode::Basic = mode {
            match (event.key, event.type_) {
                // (InputKey::Back, InputType::Short) => bt.forget_bonded_devices(),
                (InputKey::Back, InputType::Short) => mode = Mode::Mouse,
                (InputKey::Back, InputType::Long) => break,
                _ => (),
            }
            if connected {
                match (event.key, event.type_) {
                    (InputKey::Ok, InputType::Short) => {
                        let _ = bt_hid_profile.key_press(Key::Spacebar);
                    }
                    (InputKey::Ok, InputType::Long) => {
                        let _ = bt_hid_profile.key_press(Key::F);
                    }
                    (InputKey::Left, InputType::Short) => {
                        let _ = bt_hid_profile.key_press(Key::LeftArrow);
                    }
                    (InputKey::Right, InputType::Short) => {
                        let _ = bt_hid_profile.key_press(Key::RightArrow);
                    }
                    (InputKey::Left, InputType::Long) => {
                        let _ = bt_hid_profile
                            .key_press(Key::Comma | KeyMods::LeftShift);
                    }
                    (InputKey::Right, InputType::Long) => {
                        let _ = bt_hid_profile
                            .key_press(Key::Dot | KeyMods::LeftShift);
                    }
                    (InputKey::Up, InputType::Short) => {
                        let _ = bt_hid_profile.key_press(Key::Dot);
                    }
                    (InputKey::Down, InputType::Short) => {
                        let _ = bt_hid_profile.key_press(Key::Comma);
                    }
                    (InputKey::Up, InputType::Long) => {
                        let _ = bt_hid_profile.consumer_key_press(0xE9);
                    }
                    (InputKey::Down, InputType::Long) => {
                        let _ = bt_hid_profile.consumer_key_press(0xEA);
                    }
                    _ => (),
                }
                let _ = bt_hid_profile.key_release_all();
                let _ = bt_hid_profile.consumer_key_release_all();
            }
        } else if let Mode::Mouse = mode {
            match (event.key, event.type_) {
                // (InputKey::Back, InputType::Short) => bt.forget_bonded_devices(),
                (InputKey::Back, InputType::Short) => mode = Mode::Basic,
                (InputKey::Back, InputType::Long) => break,
                _ => (),
            }
            if connected {
                match (event.key, event.type_) {
                    (InputKey::Ok, InputType::Press) => {
                        let _ = bt_hid_profile.mouse_press(1);
                    }
                    (InputKey::Ok, InputType::Release) => {
                        let _ = bt_hid_profile.mouse_release(1);
                    }
                    (InputKey::Left, InputType::Press) => {
                        let _ = bt_hid_profile.mouse_move(-5, 0);
                    }
                    (InputKey::Right, InputType::Press) => {
                        let _ = bt_hid_profile.mouse_move(5, 0);
                    }
                    (InputKey::Up, InputType::Press) => {
                        let _ = bt_hid_profile.mouse_move(0, -5);
                    }
                    (InputKey::Down, InputType::Press) => {
                        let _ = bt_hid_profile.mouse_move(0, 5);
                    }
                    (InputKey::Left, InputType::Repeat) => {
                        let _ = bt_hid_profile.mouse_move(-20, 0);
                    }
                    (InputKey::Right, InputType::Repeat) => {
                        let _ = bt_hid_profile.mouse_move(20, 0);
                    }
                    (InputKey::Up, InputType::Repeat) => {
                        let _ = bt_hid_profile.mouse_move(0, -20);
                    }
                    (InputKey::Down, InputType::Repeat) => {
                        let _ = bt_hid_profile.mouse_move(0, 20);
                    }
                    _ => (),
                }
            }
        } else {
            unreachable!()
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

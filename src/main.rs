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
    furi::{message_queue::MessageQueue, time::FuriDuration},
    gui::{
        Gui,
        view_port::{InputEvent, InputKey, InputType, Orientation, ViewPort},
    },
    println,
};
use flipperzero_rt::{entry, manifest};
use flipperzero_sys as sys;

use flipper_yt_remote::{
    bt::{Bt, BtStatus, ConsumerKey, Key, KeyMods, MouseButton},
    icons,
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

        let icon = if bt_connected {
            &icons::BLE_CONNECTED
        } else {
            &icons::BLE_DISCONNECTED
        };
        canvas.draw_icon(0, 0, icon);

        match mode {
            Mode::Basic => {
                let (x, y) = (32, (128 + 15) / 2);
                canvas.draw_icon(53, 2, &icons::BASIC_MODE);
                canvas.draw_circle(x, y, 5);
                canvas.draw_icon(x - 2, y - 2, &icons::PLAY_PAUSE);
                canvas.draw_circle(x, y, 16);
                canvas.draw_icon(x - 14, y - 3, &icons::LEFT_ARROW);
                canvas.draw_icon(x + 7, y - 3, &icons::RIGHT_ARROW);
                canvas.draw_icon(x - 4, y - 13, &icons::FRAME_LEFT);
                canvas.draw_icon(x - 4, y + 7, &icons::FRAME_RIGHT);
                canvas.draw_circle(x, y, 25);
                canvas.draw_icon(x - 23, y - 2, &icons::SPEED_LEFT);
                canvas.draw_icon(x + 18, y - 2, &icons::SPEED_RIGHT);
                canvas.draw_icon(x - 3, y - 23, &icons::VOLUME_UP);
                canvas.draw_icon(x - 3, y + 18, &icons::VOLUME_DOWN);
            }
            Mode::Mouse => {
                canvas.draw_icon(54, 2, &icons::MOUSE_MODE);
            }
        }
    });
    view_port.set_input_callback(|input| {
        state
            .event_queue
            .put(input, FuriDuration::WAIT_FOREVER)
            .unwrap();
    });

    let gui = Gui::open();
    let view_port = gui.add_view_port(view_port, sys::GuiLayerFullscreen);

    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Basic = 0,
        Mouse = 1,
    }

    let mut mode = Mode::Basic;

    // TODO: re-add unpairing
    // TODO: mouse acceleration
    // TODO: better text align enum
    loop {
        state.mode.store(mode as u8, Ordering::Relaxed);
        if let Ok(event) =
            state.event_queue.get(FuriDuration::from_secs(1) / 30)
        {
            if let (InputKey::Back, InputType::Long) = (event.key, event.type_)
            {
                break;
            }

            if let Mode::Basic = mode {
                if let (InputKey::Back, InputType::Short) =
                    (event.key, event.type_)
                {
                    mode = Mode::Mouse
                }
                let key = match (event.key, event.type_) {
                    (InputKey::Ok, InputType::Short) => Some(Key::Spacebar),
                    (InputKey::Ok, InputType::Long) => Some(Key::F),
                    (InputKey::Left, InputType::Short) => Some(Key::LeftArrow),
                    (InputKey::Right, InputType::Short) => {
                        Some(Key::RightArrow)
                    }
                    (InputKey::Left, InputType::Long) => {
                        Some(Key::Comma | KeyMods::LeftShift)
                    }
                    (InputKey::Right, InputType::Long) => {
                        Some(Key::Dot | KeyMods::LeftShift)
                    }
                    (InputKey::Up, InputType::Short) => Some(Key::Dot),
                    (InputKey::Down, InputType::Short) => Some(Key::Comma),
                    _ => None,
                };
                if let Some(key) = key {
                    let _ = bt_hid_profile.key_press(key);
                    let _ = bt_hid_profile.key_release(key);
                }
                let consumer_key = match (event.key, event.type_) {
                    (InputKey::Up, InputType::Long) => {
                        Some(ConsumerKey::VolumeIncrease)
                    }
                    (InputKey::Down, InputType::Long) => {
                        Some(ConsumerKey::VolumeDecrease)
                    }
                    _ => None,
                };
                if let Some(button) = consumer_key {
                    let _ = bt_hid_profile.consumer_key_press(button);
                    let _ = bt_hid_profile.consumer_key_release(button);
                }
            } else if let Mode::Mouse = mode {
                match (event.key, event.type_) {
                    (InputKey::Back, InputType::Short) => mode = Mode::Basic,
                    (InputKey::Ok, InputType::Press) => {
                        let _ = bt_hid_profile.mouse_press(MouseButton::M1);
                    }
                    (InputKey::Ok, InputType::Release) => {
                        let _ = bt_hid_profile.mouse_release(MouseButton::M1);
                    }
                    _ => (),
                }
                let dv = match (event.key, event.type_) {
                    (InputKey::Left, InputType::Press) => Some((-5, 0)),
                    (InputKey::Right, InputType::Press) => Some((5, 0)),
                    (InputKey::Up, InputType::Press) => Some((0, -5)),
                    (InputKey::Down, InputType::Press) => Some((0, 5)),
                    (InputKey::Left, InputType::Repeat) => Some((-20, 0)),
                    (InputKey::Right, InputType::Repeat) => Some((20, 0)),
                    (InputKey::Up, InputType::Repeat) => Some((0, -20)),
                    (InputKey::Down, InputType::Repeat) => Some((0, 20)),
                    _ => None,
                };
                if let Some((dx, dy)) = dv {
                    let _ = bt_hid_profile.mouse_move(dx, dy);
                }
            } else {
                unreachable!()
            }
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

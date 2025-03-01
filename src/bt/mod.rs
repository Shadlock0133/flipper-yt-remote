// pretends to be `use flipperzero_sys as sys;`
mod sys;

use core::{
    ffi::{CStr, c_void},
    mem::ManuallyDrop,
    ops::BitOr,
    ptr::{NonNull, null_mut},
};

use alloc::boxed::Box;
use flipperzero_sys::furi::UnsafeRecord;

use crate::Error;

pub const RECORD_BT: &CStr = c"bt";

type StatusChangedCallback<'a> = dyn Fn(BtStatus) + 'a;
type ThinBox<T> = Box<Box<T>>;

pub struct Bt<'a> {
    hnd: UnsafeRecord<sys::Bt>,
    status_changed_cb: Option<ThinBox<StatusChangedCallback<'a>>>,
}

impl<'a> Bt<'a> {
    pub fn open() -> Self {
        let hnd = unsafe { UnsafeRecord::open(RECORD_BT) };
        Self {
            hnd,
            status_changed_cb: None,
        }
    }

    pub fn as_ptr(&self) -> *mut sys::Bt {
        self.hnd.as_ptr()
    }

    pub fn disconnect(&self) {
        unsafe { sys::bt_disconnect(self.as_ptr()) }
    }

    pub fn set_key_storage_path(&self, path: &CStr) {
        unsafe {
            sys::bt_keys_storage_set_storage_path(self.as_ptr(), path.as_ptr())
        }
    }

    pub fn set_default_key_storage_path(&self) {
        unsafe { sys::bt_keys_storage_set_default_path(self.as_ptr()) }
    }

    pub fn hid_profile_start(
        &self,
        mut params: sys::BleProfileHidParams,
    ) -> Result<BleProfileBase, Error> {
        let hnd = unsafe {
            sys::bt_profile_start(
                self.as_ptr(),
                sys::ble_profile_hid,
                (&raw mut params).cast(),
            )
        };
        Ok(BleProfileBase {
            hnd: NonNull::new(hnd).ok_or(Error)?,
            bt: self,
        })
    }

    // pub fn restore_default_profile(&self) -> Result<(), Error> {
    //     let res = unsafe { sys::bt_profile_restore_default(self.as_ptr()) };
    //     res.then_some(()).ok_or(Error)
    // }

    pub fn start_advertising() {
        unsafe { sys::furi_hal_bt_start_advertising() }
    }

    pub fn stop_advertising() {
        unsafe { sys::furi_hal_bt_stop_advertising() }
    }

    pub fn forget_bonded_devices(&self) {
        unsafe { sys::bt_forget_bonded_devices(self.as_ptr()) }
    }

    pub fn unset_status_changed_callback(&self) {
        unsafe {
            sys::bt_set_status_changed_callback(self.as_ptr(), None, null_mut())
        }
    }

    // TODO: this causes null ptr dereference crash for some reason
    pub fn set_status_changed_callback(&mut self, f: impl Fn(BtStatus) + 'a) {
        type CallbackStorage<'a> = Box<StatusChangedCallback<'a>>;
        unsafe extern "C" fn bt_status_changed_callback(
            status: sys::BtStatus,
            state: *mut c_void,
        ) {
            let status = match status {
                sys::BtStatusUnavailable => BtStatus::Unavailable,
                sys::BtStatusOff => BtStatus::Off,
                sys::BtStatusAdvertising => BtStatus::Advertising,
                sys::BtStatusConnected => BtStatus::Connected,
                sys::BtStatus(unknown) => BtStatus::Unknown(unknown),
            };
            let f = unsafe { &*state.cast::<CallbackStorage>() };
            f(status)
        }
        let state = self.status_changed_cb.insert(Box::new(Box::new(f)));
        let state_ptr: *mut CallbackStorage = &raw mut **state;
        unsafe {
            sys::bt_set_status_changed_callback(
                self.as_ptr(),
                Some(bt_status_changed_callback),
                state_ptr.cast(),
            )
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BtStatus {
    Unavailable,
    Off,
    Advertising,
    Connected,
    Unknown(u8),
}

pub struct BleProfileBase<'a> {
    hnd: NonNull<sys::FuriHalBleProfileBase>,
    bt: &'a Bt<'a>,
}

impl BleProfileBase<'_> {
    pub fn as_ptr(&self) -> *mut sys::FuriHalBleProfileBase {
        self.hnd.as_ptr()
    }

    /// # Safety
    /// Only call once if manually dropping
    pub unsafe fn raw_restore_default_profile(&mut self) -> Result<(), Error> {
        let res = unsafe { sys::bt_profile_restore_default(self.bt.as_ptr()) };
        res.then_some(()).ok_or(Error)
    }

    pub fn restore_default_profile(self) -> Result<(), Error> {
        let mut this = ManuallyDrop::new(self);
        unsafe { this.raw_restore_default_profile() }
    }

    pub fn key_press(&self, button: Key) -> Result<(), Error> {
        let res = unsafe {
            sys::ble_profile_hid_kb_press(self.as_ptr(), button.discriminant())
        };
        res.then_some(()).ok_or(Error)
    }

    pub fn key_release(&self, button: Key) -> Result<(), Error> {
        let res = unsafe {
            sys::ble_profile_hid_kb_release(
                self.as_ptr(),
                button.discriminant(),
            )
        };
        res.then_some(()).ok_or(Error)
    }

    pub fn key_release_all(&self) -> Result<(), Error> {
        let res = unsafe { sys::ble_profile_hid_kb_release_all(self.as_ptr()) };
        res.then_some(()).ok_or(Error)
    }

    pub fn consumer_key_press(&self, button: ConsumerKey) -> Result<(), Error> {
        let res = unsafe {
            sys::ble_profile_hid_consumer_key_press(
                self.as_ptr(),
                button.discriminant(),
            )
        };
        res.then_some(()).ok_or(Error)
    }

    pub fn consumer_key_release(
        &self,
        button: ConsumerKey,
    ) -> Result<(), Error> {
        let res = unsafe {
            sys::ble_profile_hid_consumer_key_release(
                self.as_ptr(),
                button.discriminant(),
            )
        };
        res.then_some(()).ok_or(Error)
    }

    pub fn consumer_key_release_all(&self) -> Result<(), Error> {
        let res = unsafe {
            sys::ble_profile_hid_consumer_key_release_all(self.as_ptr())
        };
        res.then_some(()).ok_or(Error)
    }

    pub fn mouse_press(&self, button: MouseButton) -> Result<(), Error> {
        let res = unsafe {
            sys::ble_profile_hid_mouse_press(
                self.as_ptr(),
                button.discriminant(),
            )
        };
        res.then_some(()).ok_or(Error)
    }

    pub fn mouse_release(&self, button: MouseButton) -> Result<(), Error> {
        let res = unsafe {
            sys::ble_profile_hid_mouse_release(
                self.as_ptr(),
                button.discriminant(),
            )
        };
        res.then_some(()).ok_or(Error)
    }

    pub fn mouse_release_all(&self) -> Result<(), Error> {
        let res =
            unsafe { sys::ble_profile_hid_mouse_release_all(self.as_ptr()) };
        res.then_some(()).ok_or(Error)
    }

    pub fn mouse_move(&self, dx: i8, dy: i8) -> Result<(), Error> {
        let res =
            unsafe { sys::ble_profile_hid_mouse_move(self.as_ptr(), dx, dy) };
        res.then_some(()).ok_or(Error)
    }

    pub fn mouse_scroll(&self, delta: i8) -> Result<(), Error> {
        let res =
            unsafe { sys::ble_profile_hid_mouse_scroll(self.as_ptr(), delta) };
        res.then_some(()).ok_or(Error)
    }
}

impl Drop for BleProfileBase<'_> {
    fn drop(&mut self) {
        let _ = unsafe { self.raw_restore_default_profile() };
    }
}

bitflags::bitflags! {
    pub struct KeyMods: u16 {
        const LeftCtrl = (1 << 8);
        const LeftShift = (1 << 9);
        const LeftAlt = (1 << 10);
        const LeftGui = (1 << 11);
        const RightCtrl = (1 << 12);
        const RightShift = (1 << 13);
        const RightAlt = (1 << 14);
        const RightGui = (1 << 15);
    }
}

impl BitOr<KeyMods> for Key {
    type Output = Key;

    fn bitor(self, rhs: KeyMods) -> Self::Output {
        Key::Other(self.discriminant() | rhs.bits())
    }
}

impl BitOr<Key> for KeyMods {
    type Output = Key;

    fn bitor(self, rhs: Key) -> Self::Output {
        Key::Other(self.bits() | rhs.discriminant())
    }
}

#[repr(u16)]
pub enum Key {
    A = 0x04,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Enter = 0x28,
    Escape,
    Backspace,
    Tab,
    Spacebar,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    RightSlash,
    Hash,
    Semicolon,
    Comma = 0x36,
    Dot,
    Slash,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    RightArrow,
    LeftArrow,
    DownArrow,
    UpArrow,
    VolumeUp = 0x80,
    VolumeDown,
    LeftCtrl = 0xE0,
    LeftShift,
    LeftAlt,
    LeftGUI,
    RightCtrl,
    RightShift,
    RightAlt,
    RightGUI,
    Other(u16),
}

impl Key {
    fn discriminant(&self) -> u16 {
        match self {
            Self::Other(other) => *other,
            _ => unsafe { *<*const _>::from(self).cast::<u16>() },
        }
    }
}

#[repr(u16)]
pub enum ConsumerKey {
    VolumeIncrease = 0xE9,
    VolumeDecrease = 0xEA,
    Other(u16),
}

impl ConsumerKey {
    fn discriminant(&self) -> u16 {
        match self {
            Self::Other(other) => *other,
            _ => unsafe { *<*const _>::from(self).cast::<u16>() },
        }
    }
}

#[repr(i8)]
pub enum MouseButton {
    M1 = 0x01,
    M2 = 0x02,
    M3 = 0x03,
    M4 = 0x04,
    M5 = 0x05,
    Other(i8),
}

impl MouseButton {
    fn discriminant(&self) -> i8 {
        match self {
            Self::Other(other) => *other,
            _ => unsafe { *<*const _>::from(self).cast::<i8>() },
        }
    }
}

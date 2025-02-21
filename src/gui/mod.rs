pub mod canvas;
pub mod view_port;

use core::ffi::CStr;

use flipperzero_sys::{self as sys, furi::UnsafeRecord};

use self::view_port::ViewPort;

pub const RECORD_GUI: &CStr = c"gui";

#[derive(Clone, Copy)]
pub struct InputEvent {
    pub type_: InputType,
    pub key: InputKey,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Font {
    Primary,
    Secondary,
    Keyboard,
    BigNumbers,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Press,
    Release,
    Short,
    Long,
    Repeat,
    Unknown(u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputKey {
    Up,
    Down,
    Right,
    Left,
    Ok,
    Back,
    Unknown(u8),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    HorizontalFlip,
    Vertical,
    VerticalFlip,
}

pub struct Gui<'a> {
    hnd: UnsafeRecord<sys::Gui>,
    view_port: Option<&'a ViewPort<'a>>,
}

impl<'a> Gui<'a> {
    pub fn open() -> Self {
        let hnd = unsafe { UnsafeRecord::open(RECORD_GUI) };
        Self {
            hnd,
            view_port: None,
        }
    }

    pub fn as_ptr(&self) -> *mut sys::Gui {
        self.hnd.as_ptr()
    }

    pub fn add_view_port(
        &mut self,
        view_port: &'a ViewPort<'a>,
        layer: sys::GuiLayer,
    ) {
        self.view_port = Some(view_port);
        unsafe {
            sys::gui_add_view_port(self.as_ptr(), view_port.as_ptr(), layer);
        }
    }
}

impl Drop for Gui<'_> {
    fn drop(&mut self) {
        if let Some(view_port) = self.view_port {
            unsafe {
                sys::gui_remove_view_port(self.as_ptr(), view_port.as_ptr())
            }
        }
    }
}

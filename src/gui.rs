use core::{
    ffi::{c_void, CStr},
    ptr::NonNull,
};

use alloc::boxed::Box;
use flipperzero_sys::{self as sys};

pub const RECORD_GUI: &CStr = c"gui";

pub struct Canvas {
    hnd: NonNull<sys::Canvas>,
}

impl Canvas {
    pub fn set_font(&self, font: Font) {
        let font = match font {
            Font::Primary => sys::FontPrimary,
            Font::Secondary => sys::FontSecondary,
            Font::Keyboard => sys::FontKeyboard,
            Font::BigNumbers => sys::FontBigNumbers,
            Font::TotalNumber => sys::FontTotalNumber,
        };
        unsafe { sys::canvas_set_font(self.hnd.as_ptr(), font) };
    }

    pub fn draw_box(&self, x: i32, y: i32, width: usize, height: usize) {
        unsafe { sys::canvas_draw_box(self.hnd.as_ptr(), x, y, width, height) };
    }

    pub fn draw_circle(&self, x: i32, y: i32, r: usize) {
        unsafe { sys::canvas_draw_circle(self.hnd.as_ptr(), x, y, r) };
    }

    pub fn draw_disc(&self, x: i32, y: i32, r: usize) {
        unsafe { sys::canvas_draw_disc(self.hnd.as_ptr(), x, y, r) };
    }

    pub fn draw_dot(&self, x: i32, y: i32) {
        unsafe { sys::canvas_draw_dot(self.hnd.as_ptr(), x, y) };
    }

    pub fn draw_frame(&self, x: i32, y: i32, width: usize, height: usize) {
        unsafe {
            sys::canvas_draw_frame(self.hnd.as_ptr(), x, y, width, height)
        };
    }

    pub fn draw_glyph(&self, x: i32, y: i32, ch: u16) {
        unsafe { sys::canvas_draw_glyph(self.hnd.as_ptr(), x, y, ch) };
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { sys::canvas_draw_line(self.hnd.as_ptr(), x1, y1, x2, y2) };
    }

    pub fn draw_rbox(
        &self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        r: usize,
    ) {
        unsafe {
            sys::canvas_draw_rbox(self.hnd.as_ptr(), x, y, width, height, r)
        };
    }

    pub fn draw_rframe(
        &self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        r: usize,
    ) {
        unsafe {
            sys::canvas_draw_rframe(self.hnd.as_ptr(), x, y, width, height, r)
        };
    }

    pub fn draw_str(&self, x: i32, y: i32, str: &CStr) {
        unsafe { sys::canvas_draw_str(self.hnd.as_ptr(), x, y, str.as_ptr()) };
    }

    pub fn draw_str_aligned(
        &self,
        x: i32,
        y: i32,
        hor: sys::Align,
        vert: sys::Align,
        str: &CStr,
    ) {
        unsafe {
            sys::canvas_draw_str_aligned(
                self.hnd.as_ptr(),
                x,
                y,
                hor,
                vert,
                str.as_ptr(),
            )
        };
    }
}

#[derive(Clone, Copy)]
pub struct InputEvent {
    pub type_: InputType,
    pub key: InputKey,
}

type DrawCallback<'a> = dyn Fn(&Canvas) + 'a;
type InputCallback<'a> = dyn Fn(InputEvent) + 'a;
type ThinBox<T> = Box<Box<T>>;

pub struct ViewPort<'a> {
    hnd: NonNull<sys::ViewPort>,
    draw_cb: Option<ThinBox<DrawCallback<'a>>>,
    input_cb: Option<ThinBox<InputCallback<'a>>>,
}

impl<'a> Default for ViewPort<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Drop for ViewPort<'a> {
    fn drop(&mut self) {
        unsafe { self.free() }
    }
}

impl<'a> ViewPort<'a> {
    pub fn new() -> Self {
        let hnd = unsafe { NonNull::new_unchecked(sys::view_port_alloc()) };
        Self {
            hnd,
            draw_cb: None,
            input_cb: None,
        }
    }

    pub fn raw(&self) -> *mut sys::ViewPort {
        self.hnd.as_ptr()
    }

    /// # Safety
    /// Only call once if manually dropping
    pub unsafe fn free(&mut self) {
        unsafe { sys::view_port_free(self.hnd.as_ptr()) }
    }

    pub fn set_draw_callback(&mut self, f: impl Fn(&Canvas) + 'a) {
        unsafe extern "C" fn draw_cb(
            canvas: *mut sys::Canvas,
            _state: *mut c_void,
        ) {
            let canvas = Canvas {
                hnd: unsafe { NonNull::new_unchecked(canvas) },
            };
            let f = unsafe { &*_state.cast::<Box<DrawCallback>>() };
            f(&canvas)
        }
        let state = self.draw_cb.insert(Box::new(Box::new(f)));
        let state_ptr: *mut Box<DrawCallback> = &raw mut **state;
        unsafe {
            sys::view_port_draw_callback_set(
                self.hnd.as_ptr(),
                Some(draw_cb),
                state_ptr.cast(),
            )
        }
    }
    pub fn set_input_callback(&mut self, f: impl Fn(InputEvent) + 'a) {
        unsafe extern "C" fn input_cb(
            input: *mut sys::InputEvent,
            _state: *mut c_void,
        ) {
            let input = unsafe { *input };
            let type_ = match input.type_ {
                sys::InputTypePress => InputType::Press,
                sys::InputTypeRelease => InputType::Release,
                sys::InputTypeShort => InputType::Short,
                sys::InputTypeLong => InputType::Long,
                sys::InputTypeRepeat => InputType::Repeat,
                sys::InputType(x) => InputType::Unknown(x),
            };
            let key = match input.key {
                sys::InputKeyUp => InputKey::Up,
                sys::InputKeyDown => InputKey::Down,
                sys::InputKeyRight => InputKey::Right,
                sys::InputKeyLeft => InputKey::Left,
                sys::InputKeyOk => InputKey::Ok,
                sys::InputKeyBack => InputKey::Back,
                sys::InputKey(x) => InputKey::Unknown(x),
            };
            let input = InputEvent { type_, key };
            let f = unsafe { &*_state.cast::<Box<InputCallback>>() };
            f(input)
        }
        let state = self.input_cb.insert(Box::new(Box::new(f)));
        let state_ptr: *mut Box<InputCallback> = &raw mut **state;
        unsafe {
            sys::view_port_input_callback_set(
                self.hnd.as_ptr(),
                Some(input_cb),
                state_ptr.cast(),
            )
        }
    }
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { sys::view_port_enabled_set(self.hnd.as_ptr(), enabled) };
    }
    pub fn set_orientation(&self, orientation: Orientation) {
        let orientation = match orientation {
            Orientation::Horizontal => sys::ViewPortOrientationHorizontal,
            Orientation::HorizontalFlip => {
                sys::ViewPortOrientationHorizontalFlip
            }
            Orientation::Vertical => sys::ViewPortOrientationVertical,
            Orientation::VerticalFlip => sys::ViewPortOrientationVerticalFlip,
        };
        unsafe {
            sys::view_port_set_orientation(self.hnd.as_ptr(), orientation)
        };
    }
    pub fn update(&self) {
        unsafe { sys::view_port_update(self.hnd.as_ptr()) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Font {
    Primary,
    Secondary,
    Keyboard,
    BigNumbers,
    TotalNumber,
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
    hnd: NonNull<sys::Gui>,
    view_port: Option<&'a ViewPort<'a>>,
}

impl<'a> Gui<'a> {
    pub fn new() -> Self {
        let hnd = unsafe {
            NonNull::new_unchecked(
                sys::furi_record_open(RECORD_GUI.as_ptr()).cast(),
            )
        };
        Self {
            hnd,
            view_port: None,
        }
    }

    pub fn add_view_port(
        &mut self,
        view_port: &'a ViewPort<'a>,
        layer: sys::GuiLayer,
    ) {
        self.view_port = Some(view_port);
        unsafe {
            sys::gui_add_view_port(self.hnd.as_ptr(), view_port.raw(), layer);
        }
    }
}

impl<'a> Drop for Gui<'a> {
    fn drop(&mut self) {
        if let Some(view_port) = self.view_port {
            unsafe {
                sys::gui_remove_view_port(self.hnd.as_ptr(), view_port.raw())
            }
        }
        unsafe { sys::furi_record_close(RECORD_GUI.as_ptr()) }
    }
}

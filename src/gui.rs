use core::{
    ffi::{c_void, CStr},
    ptr::NonNull,
};

use alloc::boxed::Box;
use flipperzero_sys as sys;

pub const RECORD_GUI: &CStr = c"gui";

pub struct Canvas {
    hnd: NonNull<sys::Canvas>,
}

impl Canvas {
    pub fn set_font(&self, font: sys::Font) {
        unsafe { sys::canvas_set_font(self.hnd.as_ptr(), font) };
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

type DrawCallback<'a> = dyn Fn(&Canvas) + 'a;
type InputCallback<'a> = dyn Fn(&sys::InputEvent) + 'a;

pub struct ViewPort<'a> {
    hnd: NonNull<sys::ViewPort>,
    draw_cb: Option<Box<DrawCallback<'a>>>,
    input_cb: Option<Box<InputCallback<'a>>>,
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
        let state = self.draw_cb.insert(Box::new(f));
        unsafe {
            // todo: state might become invalid after moving view port struct
            sys::view_port_draw_callback_set(
                self.hnd.as_ptr(),
                Some(draw_cb),
                (&raw mut *state).cast(),
            )
        }
    }
    pub fn set_input_callback(&mut self, f: impl Fn(&sys::InputEvent) + 'a) {
        unsafe extern "C" fn input_cb(
            input: *mut sys::InputEvent,
            _state: *mut c_void,
        ) {
            let f = unsafe { &*_state.cast::<Box<InputCallback>>() };
            f(unsafe { &*input })
        }
        let state = self.input_cb.insert(Box::new(f));
        unsafe {
            // todo: see [set_draw_callback]
            sys::view_port_input_callback_set(
                self.hnd.as_ptr(),
                Some(input_cb),
                (&raw mut *state).cast(),
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

use core::{ffi::c_void, ptr::NonNull};

use alloc::boxed::Box;
use flipperzero_sys as sys;

use crate::gui::{
    canvas::Canvas, InputEvent, InputKey, InputType, Orientation,
};

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

    pub fn as_ptr(&self) -> *mut sys::ViewPort {
        self.hnd.as_ptr()
    }

    /// # Safety
    /// Only call once if manually dropping
    pub unsafe fn free(&mut self) {
        unsafe { sys::view_port_free(self.as_ptr()) }
    }

    pub fn set_draw_callback(&mut self, f: impl Fn(&Canvas) + 'a) {
        unsafe extern "C" fn draw_cb(
            canvas: *mut sys::Canvas,
            _state: *mut c_void,
        ) {
            let canvas = unsafe { Canvas::from_ptr(canvas) };
            let f = unsafe { &*_state.cast::<Box<DrawCallback>>() };
            f(&canvas)
        }
        let state = self.draw_cb.insert(Box::new(Box::new(f)));
        let state_ptr: *mut Box<DrawCallback> = &raw mut **state;
        unsafe {
            sys::view_port_draw_callback_set(
                self.as_ptr(),
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
                self.as_ptr(),
                Some(input_cb),
                state_ptr.cast(),
            )
        }
    }
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { sys::view_port_enabled_set(self.as_ptr(), enabled) };
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
        unsafe { sys::view_port_set_orientation(self.as_ptr(), orientation) };
    }
    pub fn update(&self) {
        unsafe { sys::view_port_update(self.as_ptr()) }
    }
}

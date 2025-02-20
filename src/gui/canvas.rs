use core::{ffi::CStr, ptr::NonNull};

use flipperzero_sys as sys;

use crate::gui::Font;

pub struct Canvas {
    hnd: NonNull<sys::Canvas>,
}

impl Canvas {
    /// # Safety
    /// `canvas` pointer must be non-null valid pointer to [sys::Canvas]
    pub unsafe fn from_ptr(canvas: *mut sys::Canvas) -> Self {
        Self {
            hnd: unsafe { NonNull::new_unchecked(canvas) },
        }
    }

    pub fn as_ptr(&self) -> *mut sys::Canvas {
        self.hnd.as_ptr()
    }

    pub fn width(&self) -> usize {
        unsafe { sys::canvas_width(self.as_ptr()) }
    }

    pub fn height(&self) -> usize {
        unsafe { sys::canvas_height(self.as_ptr()) }
    }

    pub fn current_font_height(&self) -> usize {
        unsafe { sys::canvas_current_font_height(self.as_ptr()) }
    }

    pub fn set_font(&self, font: Font) {
        let font = match font {
            Font::Primary => sys::FontPrimary,
            Font::Secondary => sys::FontSecondary,
            Font::Keyboard => sys::FontKeyboard,
            Font::BigNumbers => sys::FontBigNumbers,
        };
        unsafe { sys::canvas_set_font(self.as_ptr(), font) };
    }

    pub fn draw_box(&self, x: i32, y: i32, width: usize, height: usize) {
        unsafe { sys::canvas_draw_box(self.as_ptr(), x, y, width, height) };
    }

    pub fn draw_circle(&self, x: i32, y: i32, r: usize) {
        unsafe { sys::canvas_draw_circle(self.as_ptr(), x, y, r) };
    }

    pub fn draw_disc(&self, x: i32, y: i32, r: usize) {
        unsafe { sys::canvas_draw_disc(self.as_ptr(), x, y, r) };
    }

    pub fn draw_dot(&self, x: i32, y: i32) {
        unsafe { sys::canvas_draw_dot(self.as_ptr(), x, y) };
    }

    pub fn draw_frame(&self, x: i32, y: i32, width: usize, height: usize) {
        unsafe { sys::canvas_draw_frame(self.as_ptr(), x, y, width, height) };
    }

    pub fn draw_glyph(&self, x: i32, y: i32, ch: u16) {
        unsafe { sys::canvas_draw_glyph(self.as_ptr(), x, y, ch) };
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { sys::canvas_draw_line(self.as_ptr(), x1, y1, x2, y2) };
    }

    pub fn draw_rbox(
        &self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        r: usize,
    ) {
        unsafe { sys::canvas_draw_rbox(self.as_ptr(), x, y, width, height, r) };
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
            sys::canvas_draw_rframe(self.as_ptr(), x, y, width, height, r)
        };
    }

    pub fn draw_str(&self, x: i32, y: i32, str: &CStr) {
        unsafe { sys::canvas_draw_str(self.as_ptr(), x, y, str.as_ptr()) };
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
                self.as_ptr(),
                x,
                y,
                hor,
                vert,
                str.as_ptr(),
            )
        };
    }

    pub fn string_width(&self, str: &CStr) -> u16 {
        unsafe { sys::canvas_string_width(self.as_ptr(), str.as_ptr()) }
    }
}

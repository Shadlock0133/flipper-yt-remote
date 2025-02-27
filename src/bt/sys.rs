// lets me pretend this module is part of flipperzero_sys crate
use flipperzero_sys as sys;
pub use sys::*;

unsafe extern "C" {
    /** Hid Keyboard Profile descriptor */
    pub unsafe static ble_profile_hid: *const sys::FuriHalBleProfileTemplate;

    /** Press keyboard button
     *
     * @param profile   profile instance
     * @param button    button code from HID specification
     *
     * @return          true on success
     */
    pub unsafe fn ble_profile_hid_kb_press(
        profile: *mut sys::FuriHalBleProfileBase,
        button: u16,
    ) -> bool;

    /** Release keyboard button
     *
     * @param profile   profile instance
     * @param button    button code from HID specification
     *
     * @return          true on success
     */
    pub unsafe fn ble_profile_hid_kb_release(
        profile: *mut sys::FuriHalBleProfileBase,
        button: u16,
    ) -> bool;

    /** Release all keyboard buttons
     *
     * @param profile   profile instance
     * @return          true on success
     */
    pub unsafe fn ble_profile_hid_kb_release_all(
        profile: *mut sys::FuriHalBleProfileBase,
    ) -> bool;

    /** Set the following consumer key to pressed state and send HID report
     *
     * @param profile   profile instance
     * @param button    key code
     */
    pub unsafe fn ble_profile_hid_consumer_key_press(
        profile: *mut sys::FuriHalBleProfileBase,
        button: u16,
    ) -> bool;

    /** Set the following consumer key to released state and send HID report
     *
     * @param profile   profile instance
     * @param button    key code
     */
    pub unsafe fn ble_profile_hid_consumer_key_release(
        profile: *mut sys::FuriHalBleProfileBase,
        button: u16,
    ) -> bool;

    /** Set consumer key to released state and send HID report
     *
     * @param profile   profile instance
     * @param button    key code
     */
    pub unsafe fn ble_profile_hid_consumer_key_release_all(
        profile: *mut sys::FuriHalBleProfileBase,
    ) -> bool;

    /** Set mouse movement and send HID report
     *
     * @param profile    profile instance
     * @param      dx    x coordinate delta
     * @param      dy    y coordinate delta
     */
    pub unsafe fn ble_profile_hid_mouse_move(
        profile: *mut sys::FuriHalBleProfileBase,
        dx: i8,
        dy: i8,
    ) -> bool;

    /** Set mouse button to pressed state and send HID report
     *
     * @param profile   profile instance
     * @param   button  key code
     */
    pub unsafe fn ble_profile_hid_mouse_press(
        profile: *mut sys::FuriHalBleProfileBase,
        button: i8,
    ) -> bool;

    /** Set mouse button to released state and send HID report
     *
     * @param profile   profile instance
     * @param   button  key code
     */
    pub unsafe fn ble_profile_hid_mouse_release(
        profile: *mut sys::FuriHalBleProfileBase,
        button: i8,
    ) -> bool;

    /** Set mouse button to released state and send HID report
     *
     * @param profile   profile instance
     * @param   button  key code
     */
    pub unsafe fn ble_profile_hid_mouse_release_all(
        profile: *mut sys::FuriHalBleProfileBase,
    ) -> bool;

    /** Set mouse wheel position and send HID report
     *
     * @param profile   profile instance
     * @param    delta  number of scroll steps
     */
    pub unsafe fn ble_profile_hid_mouse_scroll(
        profile: *mut sys::FuriHalBleProfileBase,
        delta: i8,
    ) -> bool;
}

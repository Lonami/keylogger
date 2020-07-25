pub use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};

pub type LResult = isize;
pub type WParam = usize;
pub type LParam = isize;
pub type Atom = c_ushort;

pub enum _HInstance {}
pub type HInstance = *mut _HInstance;
pub type HModule = HInstance;

pub enum _HWnd {}
pub type HWnd = *mut _HWnd;

pub enum _HIcon {}
pub type HIcon = *mut _HIcon;
pub type HCursor = HIcon;

pub enum _HBrush {}
pub type HBrush = *mut _HBrush;

pub enum _HMenu {}
pub type HMenu = *mut _HMenu;

pub enum _HRawInput {}
pub type HRawInput = *mut _HRawInput;

pub type Handle = *mut c_void;

pub type WindowProc =
    unsafe extern "C" fn(hwnd: HWnd, msg: c_uint, wparam: WParam, lparam: LParam) -> LResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Point {
    pub x: c_long,
    pub y: c_long,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct WndClassEx {
    pub cb_size: c_uint,
    pub style: c_uint,
    pub wnd_proc: Option<WindowProc>,
    pub cls_extra: c_int,
    pub wnd_extra: c_int,
    pub instance: HInstance,
    pub icon: HIcon,
    pub cursor: HCursor,
    pub background: HBrush,
    pub menu_name: *const c_char,
    pub class_name: *const c_char,
    pub icon_sm: HIcon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Msg {
    pub wnd: HWnd,
    pub message: c_uint,
    pub wparam: *const c_uint,
    pub lparam: *const c_long,
    pub time: c_ulong,
    pub pt: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawInputDevice {
    pub usage_page: c_ushort,
    pub usage: c_ushort,
    pub flags: c_ulong,
    pub hwnd_target: HWnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawInputHeader {
    pub ty: c_ulong,
    pub size: c_ulong,
    pub device: Handle,
    pub wparam: WParam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawMouse {
    pub flags: c_ushort,
    pub memory_padding: c_ushort,
    pub button_flags: c_ushort,
    pub button_data: c_ushort,
    pub raw_buttons: c_ulong,
    pub last_x: c_long,
    pub last_y: c_long,
    pub extra_information: c_ulong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawKeyboard {
    pub make_code: c_ushort,
    pub flags: c_ushort,
    pub reserved: c_ushort,
    pub vkey: c_ushort,
    pub message: c_uint,
    pub extra_information: c_ulong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawHid {
    pub size_hid: c_ulong,
    pub count: c_ulong,
    pub raw_data: [u8; 1],
}

#[repr(C)]
pub union RawInputData {
    pub mouse: RawMouse,
    pub keyboard: RawKeyboard,
    pub hid: RawHid,
}

#[repr(C)]
pub struct RawInput {
    pub header: RawInputHeader,
    pub data: RawInputData,
}

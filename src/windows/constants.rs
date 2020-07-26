use super::types::*;

pub const HWND_MESSAGE: HWnd = -3isize as HWnd;
pub const WM_CREATE: c_uint = 0x0001;
pub const WM_CLOSE: c_uint = 0x0010;
pub const WM_INPUT: c_uint = 0x00FF;

pub const MAPVK_VK_TO_CHAR: c_uint = 2;
pub const WM_KEYDOWN: c_uint = 0x0100;

// https://docs.microsoft.com/en-us/windows-hardware/drivers/hid/hid-usages#usage-page
pub const HID_USAGE_PAGE_GENERIC: c_ushort = 0x01;

pub const HID_USAGE_GENERIC_POINTER: c_ushort = 0x01;
pub const HID_USAGE_GENERIC_MOUSE: c_ushort = 0x02;
pub const HID_USAGE_GENERIC_JOYSTICK: c_ushort = 0x04;
pub const HID_USAGE_GENERIC_GAMEPAD: c_ushort = 0x05;
pub const HID_USAGE_GENERIC_KEYBOARD: c_ushort = 0x06;
pub const HID_USAGE_GENERIC_KEYPAD: c_ushort = 0x07;
pub const HID_USAGE_GENERIC_MULTI_AXIS_CONTROLLER: c_ushort = 0x08;

pub const RIDEV_NOLEGACY: c_ulong = 0x00000030;
pub const RIDEV_INPUTSINK: c_ulong = 0x00000100;
pub const RID_INPUT: c_ulong = 0x10000003;

//pub const RIM_TYPEHID: c_ulong = 2;
pub const RIM_TYPEKEYBOARD: c_ulong = 1;
pub const RIM_TYPEMOUSE: c_ulong = 0;

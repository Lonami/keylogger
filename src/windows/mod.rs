mod constants;
mod external;
mod types;

pub use constants::*;
pub use external::*;
use std::mem::{self, MaybeUninit};
use std::ops::Deref;
use std::ptr;
pub use types::*;

type Result<T> = std::result::Result<T, c_ulong>;

fn last_error<T>() -> Result<T> {
    Err(unsafe { GetLastError() })
}

pub fn get_module_handle() -> Result<HModule> {
    let instance = unsafe { GetModuleHandleA(ptr::null()) };
    if instance.is_null() {
        last_error()
    } else {
        Ok(instance)
    }
}

pub fn register_class(
    instance: HInstance,
    class_name: &'static str,
    wnd_proc: Option<WindowProc>,
) -> Result<Atom> {
    let atom = unsafe {
        RegisterClassExA(&WndClassEx {
            cb_size: mem::size_of::<WndClassEx>() as u32,
            style: 0,
            wnd_proc: Some(wnd_proc.unwrap_or(DefWindowProcA as WindowProc)),
            cls_extra: 0,
            wnd_extra: 0,
            instance,
            icon: ptr::null_mut(),
            cursor: ptr::null_mut(),
            background: ptr::null_mut(),
            menu_name: ptr::null(),
            class_name: class_name.as_ptr() as *const c_char,
            icon_sm: ptr::null_mut(),
        })
    };

    if atom == 0 {
        last_error()
    } else {
        Ok(atom)
    }
}

pub fn create_window(instance: HInstance, class_name: &'static str) -> Result<HWnd> {
    let handle = unsafe {
        CreateWindowExA(
            /* ex_style */ 0,
            /* class_name */ class_name.as_ptr() as *const c_char,
            /* window_name */ ptr::null(),
            /* style */ 0,
            /* x */ 0,
            /* y */ 0,
            /* width */ 0,
            /* height */ 0,
            /* wnd_parent */ HWND_MESSAGE,
            /* menu */ ptr::null_mut(),
            /* instance */ instance,
            /* param */ ptr::null(),
        )
    };

    if handle.is_null() {
        last_error()
    } else {
        Ok(handle)
    }
}

pub fn get_message(wnd: HWnd) -> Result<(Msg, bool)> {
    let mut msg = MaybeUninit::uninit();
    let ret = unsafe { GetMessageA(msg.as_mut_ptr(), wnd, 0, 0) };
    if ret == -1 {
        last_error()
    } else {
        let msg = unsafe { msg.assume_init() };
        Ok((msg, ret == 0))
    }
}

pub fn translate_message(msg: &Msg) -> bool {
    (unsafe { TranslateMessage(msg as *const Msg) }) != 0
}

pub fn dispatch_message(msg: &Msg) -> LResult {
    (unsafe { DispatchMessageA(msg as *const Msg) }) as isize
}

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)] // TODO release as lib?
pub enum RawInputType {
    Pointer = HID_USAGE_GENERIC_POINTER,
    Mouse = HID_USAGE_GENERIC_MOUSE,
    Joystick = HID_USAGE_GENERIC_JOYSTICK,
    GamePad = HID_USAGE_GENERIC_GAMEPAD,
    Keyboard = HID_USAGE_GENERIC_KEYBOARD,
    Keypad = HID_USAGE_GENERIC_KEYPAD,
    MultiAxisController = HID_USAGE_GENERIC_MULTI_AXIS_CONTROLLER,
}

pub fn register_raw_input_devices(hwnd: HWnd, types: &[RawInputType]) -> Result<()> {
    // TODO we never de-register it
    let mut rid = Vec::with_capacity(types.len());

    for ty in types {
        rid.push(RawInputDevice {
            usage_page: HID_USAGE_PAGE_GENERIC, // raw keyboard data only
            usage: *ty as u16,
            flags: RIDEV_NOLEGACY | RIDEV_INPUTSINK, // no legacy, system-wide
            hwnd_target: hwnd,
        });
    }

    if unsafe {
        RegisterRawInputDevices(
            rid.as_ptr(),
            rid.len() as u32,
            mem::size_of::<RawInputDevice>() as u32,
        )
    } == 0
    {
        last_error()
    } else {
        Ok(())
    }
}

pub struct RawInputValue {
    buffer: Vec<c_void>,
}

impl RawInputValue {
    pub fn keyboard(&self) -> Option<&RawKeyboard> {
        if self.header.ty == RIM_TYPEKEYBOARD {
            Some(unsafe { &self.data.keyboard })
        } else {
            None
        }
    }

    pub fn mouse(&self) -> Option<&RawMouse> {
        if self.header.ty == RIM_TYPEMOUSE {
            Some(unsafe { &self.data.mouse })
        } else {
            None
        }
    }
}

impl Deref for RawInputValue {
    type Target = RawInput;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.buffer.as_ptr() as *const Self::Target) }
    }
}

impl RawKeyboard {
    pub fn key(&self) -> char {
        std::char::from_u32(unsafe { MapVirtualKeyA(self.vkey.into(), MAPVK_VK_TO_CHAR) }).unwrap()
    }
}

pub fn get_raw_input_data(handle: HRawInput) -> std::result::Result<RawInputValue, ()> {
    let mut size = 0;
    let ret = unsafe {
        GetRawInputData(
            handle,
            RID_INPUT,
            std::ptr::null_mut(),
            &mut size,
            std::mem::size_of::<RawInputHeader>() as u32,
        )
    };
    if ret == -1i32 as u32 {
        return Err(());
    }

    let len = size as usize;
    let mut lpb = Vec::with_capacity(len);
    let ret = unsafe {
        GetRawInputData(
            handle,
            RID_INPUT,
            lpb.as_mut_ptr(),
            &mut size,
            std::mem::size_of::<RawInputHeader>() as u32,
        )
    };
    if ret != len as u32 {
        return Err(());
    }

    Ok(RawInputValue { buffer: lpb })
}

pub fn post_quit_message(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
}

pub fn default_wnd_proc(wnd: HWnd, message: u32, wparam: WParam, lparam: LParam) -> LResult {
    unsafe { DefWindowProcA(wnd, message, wparam, lparam) }
}

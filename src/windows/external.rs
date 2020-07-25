use super::types::*;

#[link(name = "USER32")]
extern "C" {
    pub fn GetLastError() -> c_ulong;

    pub fn GetModuleHandleA(module_name: *const c_char) -> HModule;

    pub fn RegisterClassExA(arg1: *const WndClassEx) -> Atom;

    pub fn CreateWindowExA(
        ex_style: c_ulong,
        class_name: *const c_char,
        window_name: *const c_char,
        style: c_ulong,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        wnd_parent: HWnd,
        menu: HMenu,
        instance: HInstance,
        param: *const c_void,
    ) -> HWnd;

    pub fn GetMessageA(
        msg: *mut Msg,
        wnd: HWnd,
        msg_filter_min: c_uint,
        msg_filter_max: c_uint,
    ) -> c_int;

    pub fn DefWindowProcA(wnd: HWnd, msg: c_uint, wparam: WParam, lparam: LParam) -> LResult;

    pub fn TranslateMessage(msg: *const Msg) -> c_int;

    pub fn DispatchMessageA(msg: *const Msg) -> c_long;

    pub fn PostQuitMessage(exit_code: c_int);

    pub fn RegisterRawInputDevices(
        raw_input_devices: *const RawInputDevice,
        num_devices: c_uint,
        cb_size: c_uint,
    ) -> c_int;

    pub fn GetRawInputData(
        raw_input: HRawInput,
        command: c_uint,
        data: *mut c_void,
        cb_size: *mut c_uint,
        cb_size_header: c_uint,
    ) -> c_uint;

    pub fn MapVirtualKeyA(code: c_uint, map_type: c_uint) -> c_uint;
}

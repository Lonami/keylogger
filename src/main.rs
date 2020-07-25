mod windows;

use std::process;

const CLASS_NAME: &str = "kl";

fn main() -> Result<(), u32> {
    let instance = windows::get_module_handle()?;
    windows::register_class(instance, CLASS_NAME, Some(wnd_proc))?;
    let wnd = windows::create_window(instance, CLASS_NAME)?;

    let ret = loop {
        let (msg, quit) = windows::get_message(wnd)?;
        if quit {
            break msg.wparam;
        }

        windows::translate_message(&msg);
        windows::dispatch_message(&msg);
    };

    process::exit(ret as isize as i32)
}

extern "C" fn wnd_proc(
    wnd: windows::HWnd,
    message: u32,
    wparam: windows::WParam,
    lparam: windows::LParam,
) -> windows::LResult {
    match message {
        windows::WM_CREATE => {
            windows::register_rid(wnd).unwrap();
        }
        windows::WM_INPUT => {
            let raw_input = match windows::get_raw_input_data(lparam as windows::HRawInput) {
                Ok(x) => x,
                Err(_) => return 0,
            };

            let keyboard = raw_input.keyboard().unwrap();
            println!(
                "{:?}, {:?} ('{}' {})",
                raw_input.header,
                keyboard,
                keyboard.key(),
                if keyboard.message == windows::WM_KEYDOWN {
                    "down"
                } else {
                    "up"
                },
            );
        }
        windows::WM_CLOSE => {
            windows::post_quit_message(0);
        }
        _ => {
            return windows::default_wnd_proc(wnd, message, wparam, lparam);
        }
    }
    0
}

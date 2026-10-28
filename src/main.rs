#![windows_subsystem = "windows"]

use custom_cmds::Env;
use rdev::grab;
use std::cell::RefCell;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_UNICODE,
    SendInput,
};

const BACKSPACE: u16 = 14;
const KEY_DOWN: u32 = KEYEVENTF_SCANCODE;
const KEY_UP: u32 = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;
const UNICODE_KEY_DOWN: u32 = KEYEVENTF_UNICODE;
const UNICODE_KEY_UP: u32 = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;

fn key_event(flags: u32, scan_code: u16) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: KEYBDINPUT {
                wVk: 0,
                wScan: scan_code,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn replace_with(text: &str, len_in_dict: usize) {
    let mut inputs = Vec::with_capacity(len_in_dict * 2 + 4);
    let backspace_down = key_event(KEY_DOWN, BACKSPACE);
    let backspace_up = key_event(KEY_UP, BACKSPACE);

    for _ in 0..len_in_dict {
        inputs.push(backspace_down);
        inputs.push(backspace_up);
    }

    for code in text.encode_utf16() {
        inputs.push(key_event(UNICODE_KEY_DOWN, code));
        inputs.push(key_event(UNICODE_KEY_UP, code));
    }

    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            size_of::<INPUT>() as i32,
        );
    }
}

fn main() {
    let env = RefCell::new(Env::new());

    if let Err(error) = grab(move |event| {
        if let Some(value) = &event.name {
            let key = *value.as_bytes().first().unwrap_or(&b'\0');
            let res = env.borrow_mut().handle_key(key);

            if let Some((text, len_in_dict)) = res {
                println!("{}", text);
                replace_with(text, len_in_dict);
                return None;
            }
        }
        Some(event)
    }) {
        println!("Error: {:?}", error);
    }
}

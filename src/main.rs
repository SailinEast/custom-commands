#![windows_subsystem = "windows"]

use custom_cmds::Env;
use rdev::grab;
use std::cell::RefCell;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput, VIRTUAL_KEY,
    VK_BACK,
};

fn key_event(vk: VIRTUAL_KEY, flags: u32, scan: u16) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk as VIRTUAL_KEY,
                wScan: scan,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn send_emoji(emoji: &str, len: usize) {
    let mut inputs = Vec::with_capacity(len * 2 + 4);
    let up = key_event(VK_BACK, KEYEVENTF_KEYUP, 0);
    let down = key_event(VK_BACK, 0, 0);

    for _ in 0..len {
        inputs.push(down);
        inputs.push(up);
    }

    for code in emoji.encode_utf16() {
        inputs.push(key_event(0, KEYEVENTF_UNICODE, code));
        inputs.push(key_event(0, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP, code));
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

            if let Some((emoji, len)) = res {
                println!("{}", emoji);
                send_emoji(emoji, len);
                return None;
            }
        }
        Some(event)
    }) {
        println!("Error: {:?}", error);
    }
}

#![crate_type="dylib"]
#![feature(asm, libc, std_misc, collections)]
#![allow(raw_pointer_derive)]

extern crate crc;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate winapi;
extern crate kernel32;
extern crate user32;

pub use interfaces::INTERFACES;
pub use offsets::OFFSETS;

mod aimbot;
mod hooks;
mod interfaces;
mod offsets;
mod sdk;
#[allow(dead_code)]
mod triggerbot;
mod vmthook;
mod utils;

fn show_popup_caption(msg: &str, caption: &str) {
    use std::ffi::CString;
    let msg = CString::new(msg).unwrap();
    let caption = CString::new(caption).unwrap();
    unsafe {
        user32::MessageBoxA(
            std::ptr::null_mut(),
            msg.as_ptr(),
            caption.as_ptr(),
            winapi::MB_OK
            );
    }
}
fn show_popup(msg: &str) {
    show_popup_caption(msg, "kappa")
}

#[allow(dead_code)]
extern "stdcall" {
    fn mouse_event(
        dwFlags: winapi::DWORD,
        dx: winapi::DWORD,
        dy: winapi::DWORD,
        dwData: winapi::DWORD,
        dwExtraInfo: winapi::ULONG_PTR
    );
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(
    _hinstDLL: winapi::HINSTANCE,
    fdwReason: winapi::DWORD,
    _lpvReserved: winapi::LPVOID) -> bool {
        if fdwReason == 1 {
            // DLL_PROCESS_ATTACH
            // spawn our setup thread asynchronously
            std::thread::spawn(setup);
        }

        true
}

fn setup() {
    unsafe {
        INTERFACES.load();
        OFFSETS.load();
        hooks::install_client();
    }
}
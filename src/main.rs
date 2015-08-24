extern crate winapi;
extern crate user32;

use winapi::windef::*;

fn main() {
    // Null terminated strings
    let text = b"Hello World\x00";
    let ptr_text = text.as_ptr() as * const i8;

    unsafe {
        user32::MessageBoxA(0 as HWND,ptr_text, ptr_text, 0);
    }
}

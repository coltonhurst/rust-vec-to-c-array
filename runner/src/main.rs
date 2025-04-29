mod mac;
mod windows;

extern crate core;
use core::ffi::c_int;

unsafe extern "C" {
    fn test() -> c_int;
}

fn main() {
    unsafe {
        println!("test(): {:?}", test());
    }
    println!("----- Mac Test -----");
    mac::mac_test();
    println!("----- Windows Test -----");
    windows::windows_test();
}

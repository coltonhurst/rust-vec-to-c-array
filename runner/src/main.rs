extern crate core;
use core::ffi::c_int;

unsafe extern "C" {
    fn test() -> c_int;
}

// #[repr(C)]
// pub struct Person {

// }

fn main() {
    println!("Hello, world!");
    
    unsafe {
        println!("test(): {:?}", test());
    }
}

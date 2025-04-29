mod list_test;

extern crate core;
use core::ffi::c_int;

unsafe extern "C" {
    fn test() -> c_int;
}

fn main() {
    println!("----- Test -----");
    unsafe {
        println!("{:?}\n", test());
    }

    println!("----- List Test -----");
    list_test::list_test();
}

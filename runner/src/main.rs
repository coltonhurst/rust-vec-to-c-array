extern crate core;
use core::ffi::c_int;

unsafe extern "C" {
    fn test() -> c_int;
    fn listPeopleOnTheBus(bus: *mut Bus, peopleCount: c_int);
}

#[repr(C)]
pub struct Person {
    name: String,
    age: i32,
}

#[repr(C)]
pub struct Bus {
    busName: String,
    people: Vec<*mut Person>,
}

fn main() {
    // test() test
    println!("test() test");
    unsafe {
        println!("test(): {:?}", test());
    }

    // listPeopleOnTheBus() test
    println!("listPeopleOnTheBus() test");
    let mut bobby = Person { name: String::from("Bobby"), age: 100 };
    let bobby_ptr: *mut Person = &mut bobby;

    let mut susan = Person { name: String::from("Susan"), age: 99 };
    let susan_ptr: *mut Person = &mut susan;

    let people: Vec<*mut Person> = vec![bobby_ptr, susan_ptr];

    let mut bus = Bus { busName: String::from("City Bus"), people };
    let bus_ptr: *mut Bus = &mut bus;

    unsafe {
        println!("listPeopleOnTheBus(): {:?}", listPeopleOnTheBus(bus_ptr, 2));
    }
}

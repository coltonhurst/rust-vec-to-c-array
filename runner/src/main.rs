extern crate core;
use core::ffi::c_int;

unsafe extern "C" {
    fn test() -> c_int;
    fn listPeopleOnTheBus(bus: *mut Bus, peopleCount: c_int);
}

#[repr(C)]
pub struct Person {
    name: *mut u8,
    age: c_int,
}

#[repr(C)]
pub struct Bus {
    busName: *mut u8,
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

    let bobby_name: *mut u8 = unsafe { String::from("Bobby").as_bytes_mut().as_mut_ptr() };
    let mut bobby = Person { name: bobby_name, age: 100 };
    let bobby_ptr: *mut Person = &mut bobby;

    let susan_name: *mut u8 = unsafe { String::from("Susan").as_bytes_mut().as_mut_ptr() };
    let mut susan = Person { name: susan_name, age: 99 };
    let susan_ptr: *mut Person = &mut susan;

    let people: Vec<*mut Person> = vec![bobby_ptr, susan_ptr];

    let bus_name: *mut u8 = unsafe { String::from("City Bus").as_bytes_mut().as_mut_ptr() };
    let mut bus = Bus { busName: bus_name, people };
    let bus_ptr: *mut Bus = &mut bus;

    unsafe {
        println!("listPeopleOnTheBus(): {:?}", listPeopleOnTheBus(bus_ptr, 2));
    }
}

/*
    This code is the same as mac.rs but the `bus_name`
    is being handled differently.
*/

extern crate core;
use core::ffi::c_int;

unsafe extern "C" {
    fn listPeopleOnTheBus(bus: *mut Bus, peopleCount: c_int);
}

#[repr(C)]
pub struct Person {
    name: *mut u8,
    age: c_int,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Bus {
    busName: *mut u8,
    people: *mut *mut Person,
}

pub fn windows_test() {
    let mut bobby_name = String::from("Bobby");
    let mut susan_name = String::from("Susan");
    let mut joey_name = String::from("Joey Appleseed");
    let bus_name = String::from("City Bus");

    let bobby_name: *mut u8 = unsafe { bobby_name.as_bytes_mut().as_mut_ptr() };
    let mut bobby = Person {
        name: bobby_name,
        age: 100,
    };
    let bobby_ptr: *mut Person = &mut bobby;

    let susan_name: *mut u8 = unsafe { susan_name.as_bytes_mut().as_mut_ptr() };
    let mut susan = Person {
        name: susan_name,
        age: 99,
    };
    let susan_ptr: *mut Person = &mut susan;

    let joey_name: *mut u8 = unsafe { joey_name.as_bytes_mut().as_mut_ptr() };
    let mut joey = Person {
        name: joey_name,
        age: 99,
    };
    let joey_ptr: *mut Person = &mut joey;

    let mut people: Vec<*mut Person> = vec![bobby_ptr, susan_ptr, joey_ptr];

    let mut bus_name = bus_name.into_bytes();
    bus_name.push(0);
    let bus_name: *mut u8 = bus_name.as_mut_ptr();
    let mut bus = Bus {
        busName: bus_name,
        people: people.as_mut_ptr(),
    };
    let bus_ptr: *mut Bus = &mut bus;

    unsafe {
        println!("listPeopleOnTheBus(): {:?}", listPeopleOnTheBus(bus_ptr, 3));
    }
}

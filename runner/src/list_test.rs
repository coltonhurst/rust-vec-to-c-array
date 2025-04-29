extern crate core;

use std::ffi::{CString, c_int};

unsafe extern "C" {
    fn listPeopleOnTheBus(bus: *mut Bus, peopleCount: c_int);
}

#[repr(C)]
pub struct Person {
    name: *const i8,
    age: c_int,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct Bus {
    busName: *const i8,
    people: *mut *mut Person,
}

pub fn list_test() {
    let bobby_name = CString::new("Bobby").expect("expect cstring");
    let susan_name = CString::new("Susan").expect("expect cstring");
    let joey_name = CString::new("Joey Appleseed").expect("expect cstring");
    let anna_name = CString::new("Anna Johnson").expect("expect cstring");
    let bus_name = CString::new("City Bus Blue").expect("expect cstring");

    // Bobby
    let mut bobby = Person {
        name: bobby_name.as_ptr(),
        age: 100,
    };
    let bobby_ptr: *mut Person = &mut bobby;

    // Susan
    let mut susan = Person {
        name: susan_name.as_ptr(),
        age: 99,
    };
    let susan_ptr: *mut Person = &mut susan;

    // Joey
    let mut joey = Person {
        name: joey_name.as_ptr(),
        age: 98,
    };
    let joey_ptr: *mut Person = &mut joey;

    // Anna
    let anna_name = anna_name.as_ptr();
    let mut anna = Person {
        name: anna_name,
        age: 97,
    };
    let anna_ptr: *mut Person = &mut anna;

    // Bus
    let mut people: Vec<*mut Person> = vec![bobby_ptr, susan_ptr, joey_ptr, anna_ptr];
    let mut bus = Bus {
        busName: bus_name.as_ptr(),
        people: people.as_mut_ptr(),
    };
    let bus_ptr: *mut Bus = &mut bus;

    // C FFI function call
    unsafe {
        listPeopleOnTheBus(bus_ptr, 4);
    }
}

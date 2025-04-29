extern crate core;
use core::ffi::c_int;

unsafe extern "C" {
    fn test() -> c_int;
    fn listPeopleOnTheBus(bus: *mut Bus, peopleCount: c_int);
}

#[repr(C)]
pub struct Person {
    name: *mut u16,
    age: c_int,
}

#[repr(C)]
pub struct Bus {
    busName: *mut u16,
    people: *mut *mut Person,
}

pub fn windows_test() {
    let bobby_name = String::from("Bobby");
    let susan_name = String::from("Susan");
    let bus_name = String::from("City Bus");

    let mut bobby_name: Vec<u16> = bobby_name.encode_utf16().collect();
    bobby_name.push(0);
    let bobby_name: *mut u16 = bobby_name.as_mut_ptr();
    let mut bobby = Person {
        name: bobby_name,
        age: 100,
    };
    let bobby_ptr: *mut Person = &mut bobby;

    let mut susan_name: Vec<u16> = susan_name.encode_utf16().collect();
    susan_name.push(0);
    let susan_name: *mut u16 = susan_name.as_mut_ptr();
    let mut susan = Person {
        name: susan_name,
        age: 100,
    };
    let susan_ptr: *mut Person = &mut susan;

    let mut people: Vec<*mut Person> = vec![bobby_ptr, susan_ptr];

    let mut bus_name: Vec<u16> = bus_name.encode_utf16().collect();
    bus_name.push(0);
    let bus_name: *mut u16 = bus_name.as_mut_ptr();
    let mut bus = Bus {
        busName: bus_name,
        people: people.as_mut_ptr(),
    };
    let bus_ptr: *mut Bus = &mut bus;

    unsafe {
        println!("listPeopleOnTheBus(): {:?}", listPeopleOnTheBus(bus_ptr, 2));
    }
}

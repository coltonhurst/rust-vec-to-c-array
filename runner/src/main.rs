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
    people: Vec<Person>,
}

fn main() {
    // test() test
    println!("test() test");
    unsafe {
        println!("test(): {:?}", test());
    }

    // listPeopleOnTheBus() test
    println!("listPeopleOnTheBus() test");
    let bobby = Person { name: String::from("Bobby"), age: 100 };
    let susan = Person { name: String::from("Susan"), age: 99 };

    let people: Vec<Person> = vec![bobby, susan];
    let mut bus = Bus { busName: String::from("City Bus"),  people };
    let bus_ptr: *mut Bus = &mut bus;

    unsafe {
        println!("test(): {:?}", listPeopleOnTheBus(bus_ptr, 2));
    }
}

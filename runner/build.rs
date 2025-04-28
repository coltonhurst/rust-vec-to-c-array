extern crate cc;

fn main() {
    cc::Build::new().file("c/lib.c").compile("lib");
}

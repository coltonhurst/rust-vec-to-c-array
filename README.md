# rust-vec-to-c-array

This is an example demonstrating how to iterate through a Rust vec in C, and the FFI between them. Specifically, passing a pointer to a list of pointers.

You can see the isolated C example in [main.c](./lib/main.c) and the Rust <-> C FFI in [main.rs](./runner/src/main.rs) and [lib.c](./runner/c/lib.c).

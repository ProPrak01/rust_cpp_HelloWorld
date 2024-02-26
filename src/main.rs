mod lib;

fn main() {
    unsafe {
        lib::ffi::hello_world();
    }
}
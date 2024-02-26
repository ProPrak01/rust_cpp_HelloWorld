#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn hello_world();
    }
}
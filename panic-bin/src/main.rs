unsafe extern "C" {
    fn c_abi_panic();                
}

unsafe extern "C-unwind" {
    fn c_unwind_abi_panic();
}

fn main() {
    let result = std::panic::catch_unwind(|| {
        unsafe { c_abi_panic(); };
    });
    match result {
        Ok(()) => println!("No panic."),
        Err(cause) => eprintln!("Panic occurred. Cause: {:?}", cause)
    }
}

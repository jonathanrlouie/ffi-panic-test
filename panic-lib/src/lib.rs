#[unsafe(no_mangle)]
pub extern "C" fn c_abi_panic() {
    panic!("panic from c_abi_panic()");
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn c_unwind_abi_panic() {
    panic!("panic from c_unwind_abi_panic()");
}

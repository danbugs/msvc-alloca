#![no_std]
#![no_main]

// must define your own
#[no_mangle]
pub(crate) extern "C" fn __chkstk() {}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use msvc_alloca::_alloca;

#[no_mangle]
pub extern "C" fn entrypoint() {
    let _ = _alloca(1024);
    loop {}
}
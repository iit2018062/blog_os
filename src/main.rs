#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// This is the entry point of the kernel, called by the bootloader.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This replaces the main function.
    loop {}
}

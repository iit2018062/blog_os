#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use blog_os::println;
use core::panic::PanicInfo;
mod serial;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Panic handler for both test and non-test scenarios
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Custom panic handler based on whether it's a test or not
    #[cfg(test)]
    {
        blog_os::test_panic_handler(info);
    }

    #[cfg(not(test))]
    {
        println!("{}", info);
        loop {}
    }
}

/// Entry point of the kernel, called by the bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// A simple test case
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

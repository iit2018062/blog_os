#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
use blog_os::{println};
mod serial;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
// Only needed outside of tests
use blog_os::task::keyboard;
use blog_os::task::{Task, simple_executor::SimpleExecutor};
extern crate alloc;
use blog_os::task::executor::Executor;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
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

/// Entry point of the kernel, called by the bootloader
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use x86_64::{structures::paging::Page, VirtAddr};
    // use blog_os::memory::translate_addr;
    // use blog_os::allocator; // new import
    // use blog_os::memory::{self, BootInfoFrameAllocator};
    //
    //
    // println!("Hello World{}", "!");
    // blog_os::init();

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe {
    //     BootInfoFrameAllocator::init(&boot_info.memory_map)
    // };
    //
    // // new
    // allocator::init_heap(&mut mapper, &mut frame_allocator)
    //     .expect("heap initialization failed");
    //
    // let x = Box::new(41);
    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);
    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());
    //
    // // create a reference counted vector -> will be freed when count reaches 0
    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    // core::mem::drop(reference_counted);
    // println!("reference count is {} now", Rc::strong_count(&cloned_reference));
    // let mut executor = SimpleExecutor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.run();
    //
    // #[cfg(test)]
    // test_main();
    //
    // println!("It did not crash!");
    // blog_os::hlt_loop();
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
    println!("It did not crash!");
    blog_os::hlt_loop();

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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();            // new
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("PANIC in test: {}\n", info);
    exit_qemu(QemuExitCode::Failed);  // Exit with failure code
    loop {}  // This ensures the function never returns
}
async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
#![no_std]
#![no_main]
use core::panic::PanicInfo;
use os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Test");
    println!("Second line");
    panic!("Some panic");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[!no_std]
#![no_main]
#![feature(custome_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
mod boot;
mod memory;
mod interrupts;
mod process;


#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("welcome to Rust OS");

    memory::init();

    interrupts::init();

    process::init();

    println!("initialization complete");

    loop {

        process::run_scheduler();

    }

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

        println!("kernel panic: {}", info);
        loop {}
}


pub mod gdt;
pub mod idt;
pub mod handlers;

use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = 
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});

pub fn init() {


        gdt::init();
        idt::init();
        unsage {PICS.lock().initialize() };

        x86_64::instructions::interrupts::enable();


}


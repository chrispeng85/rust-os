use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::interrupts::handlers:
use lazy_static::lazy_static;


lazy_static! {

    static ref IDT: InterruptDescriptorTable = {

        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handlers::breakpoint_handler);
        idt.page_fault.set_handler_fn(handlers::page_fault_handler);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(handlers::timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(handlers::keyboard_interrupt_handler);
        idt

    };

}

pub fn init() {

        IDT.load();

}




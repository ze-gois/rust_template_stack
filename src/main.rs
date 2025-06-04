#![no_std]
#![no_main]
use template;

use template::info;
pub struct Writer;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    template::info!("Welcome to the Stack Pointer template\n");

    info!("Stacking...\n");
    let stack = template::memory::Stack::from_pointer(template::Pointer(stack_pointer));
    info!("Stacked.\n");
    stack.print();

    panic!("Test 8: src/panic.rs");
}

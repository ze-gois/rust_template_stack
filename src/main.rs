#![no_std]
#![no_main]
use template;

pub struct Writer;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    template::info!("Welcome to the Stack Pointer template\n");

    let stack = template::memory::Stack::from_pointer(template::Pointer(stack_pointer));

    stack.print();

    panic!("Test 8: src/panic.rs");
}

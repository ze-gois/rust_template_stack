use crate::info;
use crate::memory;
use core::ptr;
use core::default::Default;

pub fn alloc<T: Default + Sized>(counter: usize) -> Option<*mut T> {
    let entries_mmap_pointer = unsafe {
        let size = core::mem::size_of::<T>() * counter;

        let aligned_size = (size + memory::page::SIZE - 1) & !(memory::page::SIZE - 1);

        let result = memory::mmap(
            ptr::null_mut(),
            aligned_size,
            memory::mmap::prot::READ | memory::mmap::prot::WRITE,
            memory::mmap::flag::PRIVATE | memory::mmap::flag::ANONYMOUS,
            -1,
            0,
        );

        match result {
            Ok(ptr) => ptr as *mut T,
            Err(_) => {
                info!("Failed to allocate memory\n");
                return None;
            }
        }
    };

    Some(entries_mmap_pointer)
}
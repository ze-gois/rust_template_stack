use crate::Pointer;
use crate::info;
use crate::memory;

pub mod entry;
pub use entry::Entry;

#[derive(Debug, Clone, Copy)]
pub struct Vector<'b, 'c> {
    pub pointer: Pointer,
    pub counter: usize,
    pub entries: *mut Entry<'b, 'c>,
}

impl<'b, 'c> Default for Vector<'b, 'c> {
    fn default() -> Self {
        Vector {
            pointer: Pointer::default(),
            counter: 0,
            entries: core::ptr::null_mut(),
        }
    }
}

impl<'b, 'c> Vector<'b, 'c> {
    pub fn from_pointer(pointer: Pointer) -> Self {
        let env_pointer = pointer.0 as *mut *mut u8;
        let counter = memory::misc::lengthp(env_pointer);

        let entries = match memory::alloc::<Entry<'b, 'c>>(counter as usize) {
            None => return Self::default(),
            Some(entries) => entries,
        };

        for o in 0..counter {
            unsafe { *entries.add(o as usize) = Entry::from_pointer(pointer.add(o)) };
        }

        Vector {
            pointer,
            counter,
            entries: entries,
        }
    }

    pub fn print(&self) {
        info!("Environment {{\n");
        for c in 0..self.counter {
            info!("\t{:?}\n", unsafe { *self.entries.add(c as usize) })
        }
        info!("}} Environment \n");
    }
}

pub mod entry;

pub use crate::info;
pub use entry::Entry;

#[derive(Debug)]
pub struct Vector<'a> {
    pub pointer: crate::Pointer,
    pub counter: u64,
    pub entries: *mut Entry<'a>,
}

impl<'a> Default for Vector<'a> {
    fn default() -> Self {
        Self {
            pointer: crate::Pointer::default(),
            counter: 0,
            entries: core::ptr::null_mut(),
        }
    }
}

impl<'a> Vector<'a> {
    pub fn from_pointer(pointer: crate::Pointer) -> Self {
        let counter = unsafe { *pointer.0 };
        let entries_alloc = crate::memory::alloc::<Entry<'a>>(counter as usize);
        let entries = match entries_alloc {
            None => return Self::default(),
            Some(entries) => entries,
        };

        for o in 0..counter as usize {
            unsafe { *entries.add(o as usize) = Entry::from_pointer(pointer.add(1 + o)) };
        }

        Self {
            pointer,
            counter,
            entries,
        }
    }

    pub fn print(&self) {
        info!("Arguments {{\n");
        for c in 0..self.counter {
            info!("\t{:?}\n", unsafe { *self.entries.add(c as usize) })
        }
        info!("}} Arguments \n");
    }
}

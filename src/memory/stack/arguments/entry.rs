use crate::{Pointer, memory};

#[derive(Debug, Clone, Copy)]
pub struct Entry<'e> {
    pub pointer: Pointer,
    pub value: &'e str,
}

impl<'e> Default for Entry<'e> {
    fn default() -> Self {
        Entry {
            pointer: Pointer::default(),
            value: "",
        }
    }
}

impl<'e> Entry<'e> {
    pub fn from_pointer(pointer: Pointer) -> Self {
        let length = unsafe { crate::memory::misc::length(*pointer.0 as *const u8) };
        let slice = unsafe { core::slice::from_raw_parts(*pointer.0 as *const u8, length) };
        let value = core::str::from_utf8(slice).unwrap();
        Entry { pointer, value }
    }
}

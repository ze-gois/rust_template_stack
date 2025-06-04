use crate::Pointer;

#[derive(Debug, Clone, Copy)]
pub struct Entry<'a, 'b> {
    pub pointer: Pointer,
    pub key: &'a str,
    pub value: &'b str,
}

impl Default for Entry<'_, '_> {
    fn default() -> Self {
        Entry {
            pointer: Pointer::default(),
            key: "",
            value: "",
        }
    }
}

impl<'a, 'b> Entry<'a, 'b> {
    pub fn from_pointer(pointer: Pointer) -> Self {
        let entry_pointer = unsafe { *(pointer.0 as *mut *mut u8) };

        let length = crate::memory::misc::length(entry_pointer);
        let slice = unsafe { core::slice::from_raw_parts(entry_pointer, length) };
        let full_string = core::str::from_utf8(slice).unwrap_or("");

        let key;
        let value;
        if let Some(pos) = full_string.find('=') {
            key = &full_string[..pos];
            value = &full_string[pos + 1..];
        } else {
            // If there's no '=' character, use the whole string as key
            key = full_string;
            value = "";
        }

        Entry {
            pointer,
            key,
            value,
        }
    }
}

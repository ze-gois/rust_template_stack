pub fn copy(lhs: *mut u8, rhs: *const u8, n: usize) {
    unsafe { (0..n).for_each(|i| *lhs.add(i) = *rhs.add(i)) };
}

pub fn set(lhs: *mut u8, value: u8, n: usize) {
    unsafe { (0..n).for_each(|i| *lhs.add(i) = value) };
}

pub fn length(s: *const u8) -> usize {
    let mut len = 0;

    while unsafe { *s.add(len) } != 0 {
        len += 1;
    }

    len
}

pub fn lengthp<T: Sized>(s: *mut *mut T) -> usize {
    let mut len = 0;

    while !unsafe { (*s.add(len)).is_null() } {
        len += 1;
    }

    len
}

pub fn find(s: *const u8, c: u8) -> Option<usize> {
    let mut index = 0;

    while unsafe { *s.add(index) } != 0 {
        if unsafe { *s.add(index) } == c {
            return Some(index);
        }
        index += 1;
    }

    None
}

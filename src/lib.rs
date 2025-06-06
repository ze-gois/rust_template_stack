#![no_std]
pub mod memory;
pub mod panic;
pub mod result;
pub mod syscall;

pub type _Pointer = *mut u64;
#[derive(Clone, Copy, Debug)]
pub struct Pointer(pub _Pointer);

impl Pointer {
    pub fn current() -> Self {
        let _pointer: _Pointer;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) _pointer) };
        Pointer(_pointer)
    }

    pub fn add(&self, a: usize) -> Self {
        unsafe { Pointer(self.0.add(a)) }
    }
}

impl Default for Pointer {
    fn default() -> Self {
        Pointer(core::ptr::null_mut())
    }
}

pub struct Writer;

pub fn print(msg: &str) {
    let bytes = msg.as_bytes();
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") 1usize => _,
            in("rdi") 1usize,
            in("rsi") bytes.as_ptr(),
            in("rdx") bytes.len(),
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags, readonly)
        );
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::Writer;
        let _ = write!(&mut writer, $($arg)*);
    }};
}

pub fn exit() {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 0x3c,
            in("rdi") 0usize,
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        );
    }
    unreachable!();
}

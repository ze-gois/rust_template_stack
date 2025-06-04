use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    super::info!("Panic occurred");
    let mut count = 5;
    loop {
        count -= 1;
        super::info!(".");
        if count == 0 {
            super::info!("\n");
            super::exit();
        }
    }
}

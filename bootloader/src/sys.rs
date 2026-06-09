use uefi::boot;
use core::time::Duration;

pub fn halt() -> ! {
    loop {
        boot::stall(Duration::from_secs(1));
    }
}
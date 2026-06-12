use core::time::Duration;
use uefi::boot;

pub fn halt() -> ! {
    loop {
        boot::stall(Duration::from_secs(1));
    }
}

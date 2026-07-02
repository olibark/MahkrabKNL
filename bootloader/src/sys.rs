//! Failure handler for bootloading.

use core::time::Duration;
use uefi::boot;

/// ### Loops, asking UEFI to pause aftr unrecoverable bootloader failure.
pub fn halt() -> ! {
    loop {
        boot::stall(Duration::from_secs(1));
    }
}

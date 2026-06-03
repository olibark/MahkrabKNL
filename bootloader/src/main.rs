#![no_main]
#![no_std]

extern crate alloc;

use core::time::Duration;
use log::{error, info};
use uefi::{boot, prelude::*};

mod filesys;

#[entry]
fn main() -> Status {
    if let Err(err) = uefi::helpers::init() {
        return err.status();
    }

    info!("BOOTX64.EFI started");

    match filesys::load_kernel() {
        Ok(kernel) => {
            info!(
                "read {} bytes from {}",
                kernel.bytes().len(),
                filesys::KERNEL_PATH_DISPLAY
            );
        }
        Err(err) => {
            error!("failed to load {}: {err}", filesys::KERNEL_PATH_DISPLAY);
        }
    }

    halt()
}

fn halt() -> ! {
    loop {
        boot::stall(Duration::from_secs(1));
    }
}

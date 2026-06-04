#![no_main]
#![no_std]

extern crate alloc;

use crate::{elf::validate_elf64_x86_64, filesys::KERNEL_PATH_DISPLAY};
use core::time::Duration;
use log::{error, info};
use uefi::{boot, prelude::*};

mod elf;
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
                KERNEL_PATH_DISPLAY
            );

            info!("validating {KERNEL_PATH_DISPLAY}");
            match validate_elf64_x86_64(kernel.bytes()) {
                Ok(header) => info!(
                    "validated ELF64 x86_64 kernel: entry={:#x}, program_headers={}",
                    header.e_entry, header.e_phnum
                ),
                Err(err) => error!("failed to validate {KERNEL_PATH_DISPLAY}: {err:?}"),
            }
        }
        Err(err) => {
            error!("failed to load {KERNEL_PATH_DISPLAY}: {err}");
        }
    }

    halt();
}

fn halt() -> ! {
    loop {
        boot::stall(Duration::from_secs(1));
    }
}

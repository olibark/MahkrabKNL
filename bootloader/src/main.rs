#![no_main]
#![no_std]

extern crate alloc;

use crate::{
    elf::{
        print_load_segments,
        validate_elf64_x86_64,
    },
    filesys::KERNEL_PATH_DISPLAY,
};

use log::{error, info};
use uefi::{prelude::*};

mod elf;
mod filesys;

pub(crate) mod sys;


#[entry]
fn main() -> Status {
    if let Err(err) = uefi::helpers::init() { return err.status() }

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
                Ok(header) => {
                    info!(
                        "validated ELF64 x86_64 kernel: entry={:#x}, program_headers={}",
                        header.e_entry, header.e_phnum
                    );

                    if let Err(err) = print_load_segments(kernel.bytes(), &header) {
                        error!(
                            "failed to read program headers from {KERNEL_PATH_DISPLAY}: {err:?}"
                        );
                    }
                }
                Err(err) => error!("failed to validate {KERNEL_PATH_DISPLAY}: {err:?}"),
            }
        }
        Err(err) => { error!("failed to load {KERNEL_PATH_DISPLAY}: {err}") }
    }

    sys::halt();
}

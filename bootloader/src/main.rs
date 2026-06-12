#![no_main]
#![no_std]

extern crate alloc;

use crate::{
    elf::{load_kernel_segments, validate_elf64_x86_64},
    filesys::KERNEL_PATH_DISPLAY,
};

use log::{error, info};
use uefi::prelude::*;

mod elf;
mod filesys;

pub(crate) mod sys;

#[entry]
fn main() -> Status {
    if let Err(err) = uefi::helpers::init() { return err.status() }

    info!("BOOTX64.EFI started");

    let _loaded_kernel = match filesys::load_kernel() {
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

                    match load_kernel_segments(kernel.bytes(), &header) {
                        Ok(loaded_kernel) => {
                            info!(
                                "loaded {} PT_LOAD segments; entry={:#x}",
                                loaded_kernel.segments().len(),
                                loaded_kernel.entry(),
                            );
                            for segment in loaded_kernel.segments() {
                                let flags = segment.flags();
                                info!(
                                    "PHDR[{}] loaded: mem={:#x}..{:#x} (memsz={:#x}), alloc={:#x}..{:#x} ({} pages), flags={}{}{} ({:#x})",
                                    segment.index(),
                                    segment.load_address(),
                                    segment.load_end(),
                                    segment.mem_size(),
                                    segment.allocated_address(),
                                    segment.allocated_end(),
                                    segment.allocated_pages(),
                                    flags.read_char(),
                                    flags.write_char(),
                                    flags.execute_char(),
                                    flags.bits(),
                                );
                            }
                            Some(loaded_kernel)
                        }
                        Err(err) => {
                            error!("failed to load {KERNEL_PATH_DISPLAY}: {err:?}");
                            None
                        }
                    }
                }
                Err(err) => {
                    error!("failed to validate {KERNEL_PATH_DISPLAY}: {err:?}");
                    None
                }
            }
        }
        Err(err) => {
            error!("failed to load {KERNEL_PATH_DISPLAY}: {err}");
            None
        }
    };

    sys::halt();
}

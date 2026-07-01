#![no_main]
#![no_std]

extern crate alloc;
use log::{error, info};
use uefi::prelude::*;

use crate::{
    elf::{
        load_kernel_segments, validate_elf64_x86_64, validate_kernel_entry, verify_loaded_segments,
    },
    filesys::KERNEL_PATH_DISPLAY,
};

mod elf;
mod filesys;

pub(crate) mod sys;

#[entry]
fn main() -> Status {
    if let Err(err) = uefi::helpers::init() {
        return err.status();
    }

    info!("BOOTX64.EFI started");

    let _loaded_kernel: Option<elf::load::LoadedKernel> = match filesys::load_kernel() {
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

                            for (segment, verification) in loaded_kernel
                                .segments()
                                .iter()
                                .zip(verify_loaded_segments(kernel.bytes(), &loaded_kernel))
                            {
                                let flags = segment.flags();
                                info!(
                                    "PHDR[{}] verify: mem={:#x}..{:#x} (memsz={:#x}), file_copy={}, zero_fill={}, alloc={:#x}..{:#x} ({} pages), flags={}{}{} ({:#x})",
                                    verification.index(),
                                    verification.memory_start(),
                                    verification.memory_end(),
                                    segment.mem_size(),
                                    verification.file_copy(),
                                    verification.zero_fill(),
                                    segment.allocated_address(),
                                    segment.allocated_end(),
                                    segment.allocated_pages(),
                                    flags.read_char(),
                                    flags.write_char(),
                                    flags.execute_char(),
                                    flags.bits(),
                                );
                            }

                            match validate_kernel_entry(&loaded_kernel) {
                                Ok(kernel_entry) => {
                                    info!(
                                        "transferring control to kernel entry {:#x} in executable PHDR[{}]",
                                        kernel_entry.address(),
                                        kernel_entry.segment_index(),
                                    );

                                    unsafe { (kernel_entry.function())() }
                                }
                                Err(err) => {
                                    error!("kernel entry point validation failed: {err:?}");
                                    None
                                }
                            }
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

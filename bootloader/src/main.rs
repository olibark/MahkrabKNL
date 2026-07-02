//! # UEFI bootloader entry point.
//! Loads the kernel ELF image from the EFI system partition,
//! validates its ELF64 x86-64 metadata, allocates and copies its
//! loadable segments, builds the shared kernel boot infomration,
//! and transfers control to the validated kernel entry point.
#![no_main]
#![no_std]

extern crate alloc;
use log::{error, info};
use uefi::prelude::*;

use crate::{
    boot_info::build_boot_info,
    elf::{
        load_kernel_segments, validate_elf64_x86_64, validate_kernel_entry, verify_loaded_segments,
    },
    filesys::KERNEL_PATH_DISPLAY,
};

mod boot_info;
mod elf;
mod filesys;

pub(crate) mod sys;

/// ### UEFI application entry point.
/// Initialises UEFI support, loads and validates the kernel, prepares the
/// kernels memory and boot information, then jumps to the kernel entry point.
/// 
/// The successful hand off is not expected to return. All failure paths log
/// the cause and fall through the ceiling to [`sys::halt`].
#[entry]
fn main() -> Status {
    if let Err(err) = uefi::helpers::init() {
        return err.status();
    }

    info!("BOOTX64.EFI started");
    // Retain child custody of loaded segment allocations until the kernel takes control.
    // Dropping `LoadedKernel` before the hand off frees them.
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
                            
                            // Re read each loaded segment and log the result.
                            // This is diagnostic, not part of loading path.
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
                                Ok(kernel_entry) => match build_boot_info() {
                                    Ok(boot_info) => {
                                        info!(
                                            "boot info: framebuffer={:#x} size={:#x} {}x{} stride={} pixel_format={}",
                                            boot_info.framebuffer_base,
                                            boot_info.framebuffer_size,
                                            boot_info.width,
                                            boot_info.height,
                                            boot_info.pixels_per_scan_line,
                                            boot_info.pixel_format,
                                        );
                                        info!(
                                            "transferring control to kernel entry {:#x} in executable PHDR[{}]",
                                            kernel_entry.address(),
                                            kernel_entry.segment_index(),
                                        );

                                        unsafe { (kernel_entry.function())(boot_info) }
                                    }
                                    Err(err) => {
                                        error!("failed to build boot info: {err}");
                                        None
                                    }
                                },
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

    // Every recoverable error path reaches here. *Rome*. Function should not 
    // return to UEFI after partially completed kernel loading attempt.
    sys::halt();
}

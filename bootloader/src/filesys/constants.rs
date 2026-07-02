//! # Filesystem paths used by the bootloader.

use uefi::{CStr16, cstr16};

/// Kernel ELF path as a UEFI-compatible UTF-16 string.
pub const KERNEL_PATH: &CStr16 = cstr16!("\\EFI\\BOOT\\kernel.elf");
/// Kernel ELF path as displayable text for boot logs and errors.
pub const KERNEL_PATH_DISPLAY: &str = "\\EFI\\BOOT\\kernel.elf";

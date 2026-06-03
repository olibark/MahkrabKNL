use uefi::{CStr16, cstr16};

pub const KERNEL_PATH: &CStr16 = cstr16!("\\EFI\\BOOT\\kernel.elf");
pub const KERNEL_PATH_DISPLAY: &str = "\\EFI\\BOOT\\kernel.elf";

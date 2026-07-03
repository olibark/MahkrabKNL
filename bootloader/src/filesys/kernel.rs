//! Loading of the kernel ELF file from the UEFI boot volume. 

use alloc::vec::Vec;

use super::{
    constants::KERNEL_PATH,
    error::{KernelLoadError, Result},
};
use uefi::{boot, fs::FileSystem};


/// ### The complete kernel file read from the EFI sytem partition.
/// Image is kept as raw bytes as ELF validation and segment loading
/// operate directly on the file.
#[derive(Debug)]
#[must_use = "Loaded kernel image must be retained for ELF validation and loading"]
pub struct KernelImage {
    /// Raw bytes of the kernel ELF file.
    bytes: Vec<u8>,
}

impl KernelImage {
    /// ### Creates a kernel image from previsoly read file contents.
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    /// ### Returns the raw bytes of the kernel ELF file.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// ### Loads the kernel ELF file from the boot volume.
/// The kernel is read from [`KERNEL_PATH`] on the same fileystem from which
/// the UEFI bootloader image was launched.
/// 
/// ### Errors:
///  - Returns [`KernelLoadError::OpenBootVolume`] when the boot volume cannot be
/// opened
///  - Returns [`KernelLoadError::ReadKernel`] when the krnel file cannot be read
/// from that volume.
pub fn load_kernel() -> Result<KernelImage> {
    let boot_volume = boot::get_image_file_system(boot::image_handle())
        .map_err(KernelLoadError::OpenBootVolume)?;
    let mut boot_volume = FileSystem::new(boot_volume);

    let bytes = boot_volume
        .read(KERNEL_PATH)
        .map_err(KernelLoadError::ReadKernel)?;

    Ok(KernelImage::new(bytes))
}

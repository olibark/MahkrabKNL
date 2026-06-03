use alloc::vec::Vec;

use uefi::{boot, fs::FileSystem};

use super::{
    constants::KERNEL_PATH,
    error::{KernelLoadError, Result},
};

#[derive(Debug)]
pub struct KernelImage {
    bytes: Vec<u8>,
}

impl KernelImage {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub fn load_kernel() -> Result<KernelImage> {
    let boot_volume = boot::get_image_file_system(boot::image_handle())
        .map_err(KernelLoadError::OpenBootVolume)?;
    let mut boot_volume = FileSystem::new(boot_volume);

    let bytes = boot_volume
        .read(KERNEL_PATH)
        .map_err(KernelLoadError::ReadKernel)?;

    Ok(KernelImage::new(bytes))
}

use core::fmt;

use uefi::{Error as UefiError, fs::Error as FileSystemError};

pub type Result<T> = core::result::Result<T, KernelLoadError>;

#[derive(Debug)]
pub enum KernelLoadError {
    OpenBootVolume(UefiError),
    ReadKernel(FileSystemError),
}

impl fmt::Display for KernelLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenBootVolume(err) => write!(f, "could not open boot volume: {err}"),
            Self::ReadKernel(err) => write_file_system_error(f, err),
        }
    }
}

fn write_file_system_error(f: &mut fmt::Formatter<'_>, err: &FileSystemError) -> fmt::Result {
    match err {
        FileSystemError::Io(err) => write!(
            f,
            "could not read kernel file: {} for {} (UEFI status: {})",
            err.context,
            err.path,
            err.uefi_error.status()
        ),
        FileSystemError::Path(err) => write!(f, "invalid kernel path: {err}"),
        FileSystemError::Utf8Encoding(err) => write!(f, "kernel file is not UTF-8: {err}"),
    }
}

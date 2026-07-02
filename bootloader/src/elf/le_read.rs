use crate::elf::constants::ElfLoadError;

/// ### Reads 16-bit unsigned integer from `bytes` at `offset`.
pub fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ElfLoadError> {
    let end = offset + 2;
    if end > bytes.len() {
        return Err(ElfLoadError::TooSmall);
    }

    Ok(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]))
}

/// ### Reads 32-bit unsigned integer from `bytes` at `offset`.
pub fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, ElfLoadError> {
    let end = offset + 4;
    if end > bytes.len() {
        return Err(ElfLoadError::TooSmall);
    }

    Ok(u32::from_le_bytes([
        // How french
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]))
}

/// ### Reads 64-bit unsigned integer from `bytes` at `offset`.
pub fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, ElfLoadError> {
    let end = offset + 8;
    if end > bytes.len() {
        return Err(ElfLoadError::TooSmall);
    }

    Ok(u64::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
        bytes[offset + 4],
        bytes[offset + 5],
        bytes[offset + 6],
        bytes[offset + 7],
    ]))
}

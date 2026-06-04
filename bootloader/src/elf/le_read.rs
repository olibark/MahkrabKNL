use crate::elf::constants::error::ElfLoadError;

pub fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ElfLoadError> {
    let end = offset + 2;
    if end > bytes.len() {
        return Err(ElfLoadError::TooSmall);
    }

    Ok(u16::from_le_bytes([bytes[offset], bytes[offset + 1]]))
}

pub fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, ElfLoadError> {
    let end = offset + 4;
    if end > bytes.len() {
        return Err(ElfLoadError::TooSmall);
    }

    Ok(u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]))
}

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

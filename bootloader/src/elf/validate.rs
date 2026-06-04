use crate::elf::constants::error::ElfLoadError;
use crate::elf::{
    constants::constants::{
        EiIndex, Elf64Ehdr, ElfClass, ElfData, ElfMachine, ElfMagic, ElfType, ElfVersion,
    },
    le_read::{read_u16, read_u32, read_u64},
};

pub fn validate_elf64_x86_64(bytes: &[u8]) -> Result<Elf64Ehdr, ElfLoadError> {
    const ELF64_EHDR_SIZE: usize = 64;
    const ELF64_PHDR_SIZE: u16 = 56;

    if bytes.len() < ELF64_EHDR_SIZE {
        return Err(ElfLoadError::TooSmall);
    };
    if bytes[0..4] != ElfMagic::BINDING {
        return Err(ElfLoadError::DarkMagic);
    };

    let class = ElfClass::from(bytes[EiIndex::CLASS]);
    if class != ElfClass::Elf64 {
        return Err(ElfLoadError::NotElf64);
    };

    let data = ElfData::from(bytes[EiIndex::DATA]);
    if data != ElfData::Lsb {
        return Err(ElfLoadError::UnsupportedEndian);
    };

    let ident_version = bytes[EiIndex::VERSION];
    if ident_version != 1 {
        return Err(ElfLoadError::UnsupportedVersion);
    };

    let e_type = ElfType::from(read_u16(bytes, 16)?);
    let e_machine = ElfMachine::from(read_u16(bytes, 18)?);
    let e_version = ElfVersion::from(read_u32(bytes, 20)?);
    let e_entry = read_u64(bytes, 24)?;
    let e_phoff = read_u64(bytes, 32)?;
    let e_shoff = read_u64(bytes, 40)?;
    let e_flags = read_u32(bytes, 48)?;
    let e_ehsize = read_u16(bytes, 52)?;
    let e_phentsize = read_u16(bytes, 54)?;
    let e_phnum = read_u16(bytes, 56)?;
    let e_shentsize = read_u16(bytes, 58)?;
    let e_shnum = read_u16(bytes, 60)?;
    let e_shstrndx = read_u16(bytes, 62)?;

    if e_version != ElfVersion::Current {
        return Err(ElfLoadError::UnsupportedVersion);
    };
    if e_machine != ElfMachine::X86_64 {
        return Err(ElfLoadError::UnsupportedMachine);
    };
    if e_type != ElfType::Exec && e_type != ElfType::Dyn {
        return Err(ElfLoadError::UnsupportedType);
    };
    if e_ehsize as usize != ELF64_EHDR_SIZE {
        return Err(ElfLoadError::InvalidHeaderSize);
    };
    if e_phentsize != ELF64_PHDR_SIZE {
        return Err(ElfLoadError::InvalidProgramHeaderSize);
    };

    let ph_table_start = e_phoff as usize;
    let ph_table_size = e_phentsize as usize * e_phnum as usize;
    let ph_table_end = ph_table_start
        .checked_add(ph_table_size)
        .ok_or(ElfLoadError::ProgramHeaderTableOutOfBounds)?;

    if ph_table_end > bytes.len() {
        return Err(ElfLoadError::ProgramHeaderTableOutOfBounds);
    };

    let mut ei_ident = [0u8; EiIndex::NIDENT];
    ei_ident.copy_from_slice(&bytes[..EiIndex::NIDENT]);

    Ok(Elf64Ehdr {
        ei_ident,
        e_type,
        e_machine,
        e_version,
        e_entry,
        e_phoff,
        e_shoff,
        e_flags,
        e_ehsize,
        e_phentsize,
        e_phnum,
        e_shentsize,
        e_shnum,
        e_shstrndx,
    })
}

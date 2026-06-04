use crate::elf::constants::constants::{ElfClass, ElfData, ElfMachine, ElfType, ElfVersion};

impl From<u8> for ElfClass {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Elf32,
            2 => Self::Elf64,
            other => Self::Unknown(other),
        }
    }
}

impl From<u8> for ElfData {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Lsb,
            2 => Self::Msb,
            other => Self::Unknown(other),
        }
    }
}

impl From<u32> for ElfVersion {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Current,
            other => Self::Unknown(other),
        }
    }
}

impl From<u16> for ElfType {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Rel,
            2 => Self::Exec,
            3 => Self::Dyn,
            other => Self::Unknown(other),
        }
    }
}

impl From<u16> for ElfMachine {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None,
            1 => Self::M32,
            2 => Self::Sparc,
            3 => Self::X86,
            8 => Self::Mips,
            62 => Self::X86_64,
            183 => Self::AArch64,
            other => Self::Unknown(other),
        }
    }
}

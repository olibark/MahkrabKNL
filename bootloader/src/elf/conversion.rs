use crate::elf::constants::{
    ElfClass, ElfData, ElfMachine, ElfType, ElfVersion, ProgramHeaderType,
};

/// ### Conversions from primitive types to ELF Classifications.
impl From<u8> for ElfClass {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,  // Invalid class.
            1 => Self::Elf32, // 32-bit ELF.
            2 => Self::Elf64, // 64-bit ELF.
            other => Self::Unknown(other),
        }
    }
}

/// ### Conversions from primitive types to ELF Data Encoding (endianness).
impl From<u8> for ElfData {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None, // Invalid data encoding.
            1 => Self::Lsb,  // Little-endian.
            2 => Self::Msb,  // Big-endian.
            other => Self::Unknown(other),
        }
    }
}

/// ### Conversions from primitve types to ELF Versions.
impl From<u32> for ElfVersion {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::None,    // Invalid version.
            1 => Self::Current, // Current version.
            other => Self::Unknown(other),
        }
    }
}

/// ### Conversions from primitve types to ELF Types.
impl From<u16> for ElfType {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None, // No file type.
            1 => Self::Rel,  // Relocatable file.
            2 => Self::Exec, // Executable file.
            3 => Self::Dyn,  // Shared object file.
            4 => Self::Core, // Core file.
            other => Self::Unknown(other),
        }
    }
}

/// ### Conversions from primtive types to ELF Machine Architectures.
impl From<u16> for ElfMachine {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None,      // No machine.
            1 => Self::M32,       // AT&T WE 32100.
            2 => Self::Sparc,     // SPARC.
            3 => Self::X86,       // Intel 80386.
            8 => Self::Mips,      // MIPS I Architecture.
            62 => Self::X86_64,   // AMD x86-64 architecture.
            183 => Self::AArch64, // ARM Aarch64 architecture.
            other => Self::Unknown(other),
        }
    }
}

/// ### Conversions from primitive types to ELF Program Header Types.
impl From<u32> for ProgramHeaderType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Null,    // Null program header.
            1 => Self::Load,    // Loadable segment.
            2 => Self::Dynamic, // Dynamic linking information.
            3 => Self::Interp,  // Interpreter information.
            4 => Self::Note,    // Note information.
            5 => Self::Shlib,   // Shared object file.
            6 => Self::Phdr,    // Program header.
            7 => Self::Tls,     // Thread-local storage.
            other => Self::Unknown(other),
        }
    }
}

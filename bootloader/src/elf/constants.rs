//! # ELF constants, decoded header types, and loader errors.
/*
                  .;;,.
                  ; '" ;\ \//
                 \|a (a|7 \//
                 j| ..  | ||/
                //'.--.')\-,/
              .-||- '' ||/  `-.
             ;  | \ |/ |/ L.  ,|
             f\ |\| Y  || \ '._\
            j | \|     (| |   | |
           |  L_\         L.__: |
            \(  '-.,-,    |   ; |
             |'-.'.L_rr>  f--f  |
.-=,,______,--------- J-. ;  ;__ 
   ``"-,__   |  |      h  |  f  '--.__
       `--;;--,_       h  f-j   |   __;==-.
            / `-''-,,__J,'  \_..--:'-'     '
            | |    `' --L7//'-'`|
            | ,     ||  h    |  (
            | ;     | \ J    j   |
            | L__   | |  L_.'    |
            |   |'-.| L.'h  |  : |
            |;  \     |  J ; : : |
            | :  (    \  'L| : : |
            | ;   \'.--|    \  : |
            | | : \    \-, /`\ : |
            L-'-;__\   \\ '  | | |
                    ;   \\   |'L_j
                    _>  _|   |
    OB: 04/06/26   <___/ /-  \
                        /    /
                        '---'
*/
#![allow(dead_code)]

use uefi::Status;

/// Address type for 64-bit ELF files.
pub type Elf64Addr = u64;
/// 64-bit file offset.
pub type Elf64Off = u64;
/// 32-bit unsigned ELF word.
pub type Elf64Word = u32;
/// 16-bit unsigned half ELF word.
pub type Elf64Half = u16;

/// ### ELF constants and types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfClass {
    /// I|U ELF class.
    None = 0,
    /// 32-bit ELF file.
    Elf32 = 1,
    /// 64-bit ELF file.
    Elf64 = 2,
    /// Unrecognised.
    Unknown(u8),
}

/// ### ELF data encoding (endianness).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfData {
    /// I|U byte order.
    None = 0,
    /// Little-endian.
    Lsb = 1,
    /// Big-endian.
    Msb = 2,
    /// Unrecognised byte-order.
    Unknown(u8),
}

/// ### ELF version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ElfVersion {
    /// I|U ELF version.
    None = 0,
    /// Current ELF spec version.
    Current = 1,
    /// Unrecognised ELF version.
    Unknown(u32),
}

/// ### ELF file type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfType {
    /// None specified.
    None = 0,
    /// Relocatable file.
    Rel = 1,
    /// Executable file.
    Exec = 2,
    /// Shared object file.
    Dyn = 3,
    /// Core file.
    Core = 4,
    /// Unrecognised ELF file.
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ElfMachine {
    /// Unspecified architecture.
    None = 0,
    /// AT&T WE 32100.
    M32 = 1,
    /// SPARC.    
    Sparc = 2,
    /// Intel 80386.
    X86 = 3,
    /// MIPS.
    Mips = 8,
    /// AMD x86-64.
    X86_64 = 62,
    /// ARM 64.
    AArch64 = 183,
    /// Unrecognised architecture.
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ProgramHeaderType {
    /// Unused entry.
    Null = 0,
    /// Loadable segment.
    Load = 1,
    /// Dynamic linking information.
    Dynamic = 2,
    /// Program interpreter path.
    Interp = 3,
    /// Auxillery note data.
    Note = 4,
    /// Reserved shared-library segment type.
    Shlib = 5,
    /// Program header table.
    Phdr = 6,
    /// Thread-local storage segment.
    Tls = 7,
    /// Unrecognised program header type.
    Unknown(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgramHeaderFlags(pub Elf64Word);

impl ProgramHeaderFlags {
    /// Executable segment permission.
    const EXECUTE: Elf64Word = 0x1;
    /// Writable segment permission.
    const WRITE: Elf64Word = 0x2;
    /// Readable segment permission.
    const READ: Elf64Word = 0x4;

    /// ### Raw program header flag bits.
    pub fn bits(self) -> Elf64Word {
        self.0
    }

    /// ### Returns whether the segment is readable.
    pub fn read(self) -> bool {
        self.0 & Self::READ != 0
    }

    /// ### Returns whether the segment is writable.
    pub fn write(self) -> bool {
        self.0 & Self::WRITE != 0
    }

    /// ### Returns whether the segment is executable.
    pub fn execute(self) -> bool {
        self.0 & Self::EXECUTE != 0
    }

    /// ### Display character for the readable flag.
    pub fn read_char(self) -> char {
        if self.read() { 'R' } else { '-' }
    }

    /// ### Display character for the writable flag.
    pub fn write_char(self) -> char {
        if self.write() { 'W' } else { '-' }
    }

    /// ### Display character for the executable flag.
    pub fn execute_char(self) -> char {
        if self.execute() { 'X' } else { '-' }
    }
}

/// ### ELF magic byte constants.
pub struct ElfMagic;
impl ElfMagic {
    /// Magic number.
    pub const MAG0: u8 = 0x7f;
    /// E.
    pub const MAG1: u8 = b'E';
    /// L.
    pub const MAG2: u8 = b'L';
    /// F.
    pub const MAG3: u8 = b'F';
    /// ELF magic numbers as a byte array.
    pub const BINDING: [u8; 4] = [Self::MAG0, Self::MAG1, Self::MAG2, Self::MAG3];
}

/// ### Indexes into the ELF `e_ident` byte array.
pub struct EiIndex;
impl EiIndex {
    /// Magic-bytes[0].
    pub const MAG0: usize = 0;
    /// Magic-bytes[1].
    pub const MAG1: usize = 1;
    /// Magic-bytes[2].
    pub const MAG2: usize = 2;
    /// Magic-bytes[3].
    pub const MAG3: usize = 3;
    /// Class: 64/32-bit.
    pub const CLASS: usize = 4;
    /// Byte order.
    pub const DATA: usize = 5;
    /// Format version.
    pub const VERSION: usize = 6;
    /// Target operating system ABI (Application Binary Interface).
    pub const OSABI: usize = 7;
    /// ABI version.
    pub const ABIVERSION: usize = 8;
    /// Total size of e_ident in bytes.
    pub const NIDENT: usize = 16;
}

/// ### Decoded ELF64 file header.
pub struct Elf64Ehdr {
    /// ELF identity bytes.
    pub ei_ident: [u8; EiIndex::NIDENT],
    /// ELF file type.
    pub e_type: ElfType,
    /// Target machine architecture.
    pub e_machine: ElfMachine,
    /// ELF object file version.
    pub e_version: ElfVersion,
    /// Kernel entry virtual address.
    pub e_entry: Elf64Addr,
    /// Program header table file offset.
    pub e_phoff: Elf64Off,
    /// Section header table file offset.
    pub e_shoff: Elf64Off,
    /// Processor-specific flags.
    pub e_flags: Elf64Word,
    /// ELF header size in bytes.
    pub e_ehsize: Elf64Half,
    /// Program header table entry size in bytes.
    pub e_phentsize: Elf64Half,
    /// Number of program header table entries.
    pub e_phnum: Elf64Half,
    /// Section header table entry size in bytes.
    pub e_shentsize: Elf64Half,
    /// Number of section header table entries.
    pub e_shnum: Elf64Half,
    /// Section header string table index.
    pub e_shstrndx: Elf64Half,
}

/// ### Decoded ELF64 program header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf64Phdr {
    /// Segment type.
    pub p_type: ProgramHeaderType,
    /// Segment permission flags.
    pub p_flags: ProgramHeaderFlags,
    /// Segment file offset.
    pub p_offset: Elf64Off,
    /// Segment virtual address.
    pub p_vaddr: Elf64Addr,
    /// Segment physical address.
    pub p_paddr: Elf64Addr,
    /// Number of file-backed bytes in the segment.
    pub p_filesz: u64,
    /// Number of bytes occupied by the segment in memory.
    pub p_memsz: u64,
    /// Segment alignment requirement.
    pub p_align: u64,
}

/// ### ELF validation and loading errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfLoadError {
    /// Too small to contain ELF header.
    TooSmall,
    /// ELF magic bytes do not match.
    DarkMagic,
    /// Not 64-bit ELF.
    NotElf64,
    /// Unsepported endianness.
    UnsupportedEndian,
    /// Unsupported ELF version.
    UnsupportedVersion,
    /// Not supported by loader.
    UnsupportedType,
    /// Target CPU unsupported.
    UnsupportedMachine,
    /// ELF header has unexpected size.
    InvalidHeaderSize,
    /// Program header has unexpected size.
    InvalidProgramHeaderSize,
    /// Program header table exceeds file.
    ProgramHeaderTableOutOfBounds,
    /// Segment file data exceeds memory size.
    SegmentFileLargerThanMemory,
    /// Segment file range exceeds ELF file.
    SegmentFileRangeOutOfBounds,
    /// Segment size calculation overflowed.
    SegmentSizeOverflow,
    /// Segment address calculation overflowed.
    SegmentAddressOverflow,
    /// Segment resolved to address zero.
    NullLoadAddress,
    /// UEFI page allocation failure.
    AllocatePages(Status),
}

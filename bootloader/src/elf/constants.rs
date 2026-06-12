/*
OB: 04/06/26
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

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Word = u32;
pub type Elf64Half = u16;

use uefi::Status;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfClass {
    None = 0,
    Elf32 = 1,
    Elf64 = 2,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfData {
    None = 0,
    Lsb = 1, // Little-endian
    Msb = 2, // Big-endian
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ElfVersion {
    None = 0,
    Current = 1,
    Unknown(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfType {
    None = 0,
    Rel = 1,  // Relocatable file
    Exec = 2, // Executable file
    Dyn = 3,  // Shared objet file
    Core = 4, // Core file
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ElfMachine {
    None = 0,
    M32 = 1,
    Sparc = 2,
    X86 = 3,
    Mips = 8,
    X86_64 = 62,
    AArch64 = 183,
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ProgramHeaderType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    Shlib = 5,
    Phdr = 6,
    Tls = 7,
    Unknown(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgramHeaderFlags(pub Elf64Word);

impl ProgramHeaderFlags {
    const EXECUTE: Elf64Word = 0x1;
    const WRITE: Elf64Word = 0x2;
    const READ: Elf64Word = 0x4;

    pub fn bits(self) -> Elf64Word {
        self.0
    }

    pub fn read(self) -> bool {
        self.0 & Self::READ != 0
    }

    pub fn write(self) -> bool {
        self.0 & Self::WRITE != 0
    }

    pub fn execute(self) -> bool {
        self.0 & Self::EXECUTE != 0
    }

    pub fn read_char(self) -> char {
        if self.read() { 'R' } else { '-' }
    }

    pub fn write_char(self) -> char {
        if self.write() { 'W' } else { '-' }
    }

    pub fn execute_char(self) -> char {
        if self.execute() { 'X' } else { '-' }
    }
}

pub struct ElfMagic;
impl ElfMagic {
    pub const MAG0: u8 = 0x7f; // elf magic number
    pub const MAG1: u8 = b'E'; // \
    pub const MAG2: u8 = b'L'; //  }-> ELF
    pub const MAG3: u8 = b'F'; // /
    pub const BINDING: [u8; 4] = [Self::MAG0, Self::MAG1, Self::MAG2, Self::MAG3];
}

pub struct EiIndex;
impl EiIndex {
    pub const MAG0: usize = 0;
    pub const MAG1: usize = 1;
    pub const MAG2: usize = 2;
    pub const MAG3: usize = 3;
    pub const CLASS: usize = 4;
    pub const DATA: usize = 5;
    pub const VERSION: usize = 6;
    pub const OSABI: usize = 7;
    pub const ABIVERSION: usize = 8;
    pub const NIDENT: usize = 16;
}

pub struct Elf64Ehdr {
    pub ei_ident: [u8; EiIndex::NIDENT],
    pub e_type: ElfType,
    pub e_machine: ElfMachine,
    pub e_version: ElfVersion,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf64Phdr {
    pub p_type: ProgramHeaderType,
    pub p_flags: ProgramHeaderFlags,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfLoadError {
    TooSmall,
    DarkMagic,
    NotElf64,
    UnsupportedEndian,
    UnsupportedVersion,
    UnsupportedType,
    UnsupportedMachine,
    InvalidHeaderSize,
    InvalidProgramHeaderSize,
    ProgramHeaderTableOutOfBounds,
    SegmentFileLargerThanMemory,
    SegmentFileRangeOutOfBounds,
    SegmentSizeOverflow,
    SegmentAddressOverflow,
    NullLoadAddress,
    AllocatePages(Status),
}

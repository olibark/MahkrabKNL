/*

    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⠤⠤⠤⠤⠤⢤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢺⡄⠀⠀⠀⠀⠀⠀⠈⠉⠒⠦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢳⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣳⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣆⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⡃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣜⣄⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⡀⠀⠘⢾⣆⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣊⠁⣀⣀⣀⣀⣀⠤⠤⠔⠒⠒⠚⠉⠉⠉⠉⠉⠉⠉⠁⠀⠀⠉⠓⠦⡀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⡠⠒⠉⠁⠀⠈⠉⠉⠉⠁⠀⣀⣀⣀⡤⠤⠤⠴⠒⠒⠲⠤⠤⢤⣀⡀⠀⠀⠀⠀⣈⡆⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⣹⢤⢤⣀⣀⣀⣠⣤⠴⢲⣏⣽⠧⣄⠀⠀⠀⠀⠀⠀⢰⡶⡦⢤⡀⠹⡟⢖⠒⠒⢻⡀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⢸⡇⢼⠁⠀⡽⣿⣼⠃⠀⠹⠿⠷⠟⠛⠃⠀⠀⠀⠀⠀⠈⠉⠛⠚⠾⠆⣷⠸⡅⠙⢦⡇⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠸⡇⢈⢤⡞⡝⣫⢻⠀⠀⠀⠀⢰⣶⣤⣄⠀⠀⠀⠀⠀⠀⣴⣶⣂⠀⠀⢸⠀⢻⡄⣾⡇⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠹⣞⣘⣾⣕⡿⡸⠀⠀⠀⠀⠺⠿⣿⠗⠀⢀⣠⠤⢄⡘⢿⣿⠟⠀⠀⠈⡆⢸⣟⡿⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⣾⣟⡾⡞⠀⡇⠀⠀⠀⠀⠀⠀⠉⠀⣠⠏⠀⠀⠀⠑⣟⠉⠀⠀⠀⠀⢱⠀⣯⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⢀⣠⣿⣼⣘⡁⠀⢧⡀⠀⠀⠀⠀⠀⢀⣠⡿⣖⠦⠤⢖⣲⢯⣀⠀⠀⠀⠀⢸⠀⢸⡄⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⣠⠖⠋⠁⠀⠀⠀⠉⠓⢤⠙⠒⢒⣒⣒⡿⣝⡯⠋⠁⣀⣀⡀⠉⠻⣯⢍⠑⠒⢴⡚⠒⢸⠃⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⢰⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⣷⠀⠈⠩⠓⠋⣁⡠⠴⣏⣁⣈⣩⠷⠦⣈⡚⠽⢖⠤⠤⣠⣿⡀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠘⣷⡄⠀⠀⠀⠀⠀⠀⠀⣠⠏⢑⡖⠒⠒⣫⠏⠀⠀⠀⠸⡆⠀⠀⠀⠈⢉⠒⠒⠒⠒⠁⢸⠙⣆⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⢀⣴⡿⠚⠦⠤⠤⠤⠤⢤⠞⠋⢠⠋⠀⣰⢲⠇⠀⠀⠀⠀⠀⢹⠀⠀⢀⠀⠀⠳⡀⠀⠀⠀⣠⡇⠘⣄⠀ ⠀⠀⠀⠀⠀
    ⣠⡟⡞⠀⠀⠀⠀⠀⠀⠀⣼⡃⢀⡟⠀⣰⠃⣽⡄⠀⠀⠀⠀⠀⠈⣆⠀⠀⠀⠀⠀⢣⠀⠀⠀⢻⢿⣯⣽⠽ ⠶⣄⠀⠀⠀
    ⢽⡇⠇⠀⠐⠒⠒⠒⠒⠋⠁⣻⡎⠁⢠⠃⠀⢹⡇⠀⠀⠀⠀⠀⢠⠏⠀⠀⣇⠀⠀⢸⠀⠀⢴⡾⠀⢷⡉⢱ ⡀⠈⠳⣄⠀
    ⢸⠀⡇⠀⠀⢤⣀⣀⣀⣀⠔⢹⡧⡄⠸⡄⠀⢸⡇⠀⠀⠀⠀⢀⡞⠀⠀⠀⣿⠀⠀⡎⠀⢰⣸⢧⠉⠙⣟⠊ ⢁⣴⠆⠈⣇
    ⠘⣇⠹⣆⠀⣀⡈⠉⠉⢀⣠⡛⠀⠙⠲⣏⠀⠘⠃⠀⠀⠀⠀⡞⠀⠀⠀⢠⡏⠀⠀⡇⣄⣴⠋⢈⡷⠀⣿⠀ ⠀⣻⡤⢴⡏
    ⠀⠘⠦⣈⣣⣌⣉⠒⠚⠉⣸⠁⠀⠀⠀⠘⢦⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⡞⠀⠀⠀⢁⡿⠁⠀⣼⡀⠀⠀⢀ ⣸⣏⠀⣠⠇
    ⠀⠀⠀⠀⢉⡏⠉⠉⡹⠉⡇⠀⠀⠀⠀⠀⠀⠙⢦⠀⠀⠀⢸⠀⠀⠀⡸⠁⠀⠀⣺⠋⠀⠀⣴⠃⠙⠢⣄⣈ ⣿⠉⣿⡟⠀
    ⠀⠀⠀⠀⡜⠀⠀⢠⠃⢰⠁⠀⠀⠀⠀⠀⠀⠀⠈⠣⣄⡀⠈⠀⢀⠜⠁⢀⡤⠴⠋⠀⢀⣜⠁⠀⠀⠀⠀⠻ ⠼⠿⠉⠀⠀
    ⠀⠀⠀⢀⡇⠀⢀⠏⠀⠈⡣⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠢⡀⢋⡤⠔⠊⢀⣀⡤⠖⠁⢹⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⣾⠀⠀⡜⠀⢠⡏⠀⠀⠉⠉⠉⠒⠒⠒⠤⠤⠤⡤⠤⠽⠟⠒⠒⠉⠉⠀⠀⢀⡤⠊⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⡟⠀⣰⠁⠀⢨⠇⠦⠤⣄⣀⣀⠀⠀⠀⠀⠀⢀⣷⠀⠀⠀⠀⠀⣀⣀⠤⠚⠙⣇⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⠀⡇⢀⡏⠀⠀⡎⠀⠀⠀⠀⠀⠉⠉⠑⠒⠒⠚⠉⡇⠐⠒⠒⠉⠉⠉⠀⠀⠀⠀⠙⠦⣀⡀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⢠⡇⠸⠀⢀⡼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠒⢤⡀⠀ ⠀⠀⠀⠀⠀
    ⠀⠀⢸⡇⠁⣠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣆ ⠀⠀⠀⠀⠀
    ⠀⠀⢸⡇⣰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈ ⣇⠀⠀⠀⠀
    ⠀⠀⢸⡇⢻⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣔⡋⠙⠒⠦⣄⠀  OB: 04/06/26⠀⠀ ⣽⠀⠀⠀⠀
    ⠀⠀⠘⢧⣀⣷⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⢏⡼⠛⠒⠤⢄⣀⠉⠒⠢⠤⢄⣀⣀⣀⣀⣀⣀⣀⠤⢜⡿⠀⠀⠀⠀
    ⠀⠀⠀⠀⠈⢿⣈⠙⠒⠦⠤⠤⠤⠤⠤⠤⠔⢊⡵⠋⠀⠀⠀⠀⠀⠈⠉⠒⠒⠢⠤⠤⠤⠤⠤⠤⠤⠤⠴⠚⠁⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠉⠙⠒⠒⠒⠒⠒⠒⠒⠒⠂⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⠀⠀⠀

         ____  __    ____       ___  __   __ _  ____  ____  ____
        (  __)(  )  (  __)___  / __)/  \ (  ( \/ ___)(_  _)/ ___)
        ) _) / (_/\ ) _)(___)( (__(  O )/    /\___ \  )(  \___ \
       (____)\____/(__)       \___)\__/ \_)__)(____/ (__) (____/


*/

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Word = u32;
pub type Elf64Half = u16;

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

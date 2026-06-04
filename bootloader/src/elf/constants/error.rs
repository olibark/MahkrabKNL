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
}

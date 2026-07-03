//! # Iteration and decofing for ELF64 program headers.
//!
//! [`ProgramHeaders`] validates the prohram-header table bounds once, then
//! yields each entry as an [`Elf64Phdr`] decoded from the ELF bytes.
//! Individual entries are returned as [`Result`]s because decoding may still
//! fail if an entry is malformed.

use crate::elf::{
    constants::{Elf64Ehdr, Elf64Phdr, ElfLoadError, ProgramHeaderFlags, ProgramHeaderType},
    le_read::{read_u32, read_u64},
};

/// ### An iterator over the program header teable of the ELF64 file.
/// The iterator borrows the complete ELF file and decodes one program header
/// per call to [`Iterator::next`].
///
/// The program header table range is validated when constructed. Each yielded
/// entry is still able to throw its toys, as its individual fields must be decoded
/// from the underlying byte pizza.
pub struct ProgramHeaders<'a> {
    /// Raw ELF bytes to decode.
    bytes: &'a [u8],
    /// Byte offset of the next program header entry.
    next_offset: usize,
    /// Number of program header entriews remaining.
    remaining: u16,
    /// Size, in bytes, of one program header table entry.
    entry_size: usize,
}

impl<'a> ProgramHeaders<'a> {
    /// ### Creates an iterator over the ELF files prgram header table.
    /// The table location and total table size are checked against `bytes`
    /// before the iterator is returned.
    ///
    /// ### Errors:
    ///  - Returns [`ElfLoadError::ProgramHeaderTableOutOfBounds`] when:
    ///    - `e_phoff` cannot be represented as [`usize`];
    ///    - Calculating the table size overflows;
    ///    - Calculating the table end overflows;
    ///    - The table exceeds beyonf the ELF file bytes.
    pub fn new(bytes: &'a [u8], header: &Elf64Ehdr) -> Result<Self, ElfLoadError> {
        let next_offset = usize::try_from(header.e_phoff)
            .map_err(|_| ElfLoadError::ProgramHeaderTableOutOfBounds)?;

        let entry_size = usize::from(header.e_phentsize);
        let table_size = entry_size
            .checked_mul(usize::from(header.e_phnum))
            .ok_or(ElfLoadError::ProgramHeaderTableOutOfBounds)?;
        let table_end = next_offset
            .checked_add(table_size)
            .ok_or(ElfLoadError::ProgramHeaderTableOutOfBounds)?;

        if table_end > bytes.len() {
            return Err(ElfLoadError::ProgramHeaderTableOutOfBounds);
        }

        Ok(Self {
            bytes,
            next_offset,
            remaining: header.e_phnum,
            entry_size,
        })
    }
}

impl Iterator for ProgramHeaders<'_> {
    /// ### A decoded ELF64 program header, or an error are encountered :( when decoding it.
    type Item = Result<Elf64Phdr, ElfLoadError>;

    /// ### Decodes and returns the next program header entry.
    /// Returns [`None`] once all entrues declared by `e_phnum` where yielded.
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let offset = self.next_offset;

        // Advance first so each call consumes exactly one table entry,
        // including an enrry thats decoding later returns an error.
        self.next_offset += self.entry_size;
        self.remaining -= 1;

        Some(read_program_header(self.bytes, offset))
    }
}

/// ### Decodes one ELF64 program header begining at `offset`.
/// ELF64 program headers stored in little-endian byte order and
/// occupy fixed offsets within the entry.
fn read_program_header(bytes: &[u8], offset: usize) -> Result<Elf64Phdr, ElfLoadError> {
    Ok(Elf64Phdr {
        p_type: ProgramHeaderType::from(read_u32(bytes, offset)?),
        p_flags: ProgramHeaderFlags(read_u32(bytes, offset + 4)?),
        p_offset: read_u64(bytes, offset + 8)?,
        p_vaddr: read_u64(bytes, offset + 16)?,
        p_paddr: read_u64(bytes, offset + 24)?,
        p_filesz: read_u64(bytes, offset + 32)?,
        p_memsz: read_u64(bytes, offset + 40)?,
        p_align: read_u64(bytes, offset + 48)?,
    })
}

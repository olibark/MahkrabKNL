use crate::elf::{
    constants::{Elf64Ehdr, Elf64Phdr, ElfLoadError, ProgramHeaderFlags, ProgramHeaderType},
    le_read::{read_u32, read_u64},
};

pub struct ProgramHeaders<'a> {
    bytes: &'a [u8],
    next_offset: usize,
    remaining: u16,
    entry_size: usize,
}

impl<'a> ProgramHeaders<'a> {
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
    type Item = Result<Elf64Phdr, ElfLoadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let offset = self.next_offset;
        self.next_offset += self.entry_size;
        self.remaining -= 1;

        Some(read_program_header(self.bytes, offset))
    }
}

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

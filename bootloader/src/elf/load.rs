use alloc::vec::Vec;
use core::ptr;

use log::info;

use crate::elf::{
    constants::{Elf64Ehdr, Elf64Phdr, ElfLoadError, ProgramHeaderFlags, ProgramHeaderType},
    pages::{align_down_to_page, align_up_to_page, page_offset, LoadedPages},
    program_header::ProgramHeaders,
};

pub struct LoadedKernel {
    entry: u64,
    segments: Vec<LoadedSegment>,
}

impl LoadedKernel {
    pub fn entry(&self) -> u64 {
        self.entry
    }

    pub fn segments(&self) -> &[LoadedSegment] {
        &self.segments
    }
}

pub struct LoadedSegment {
    index: usize,
    header: Elf64Phdr,
    pages: LoadedPages,
    load_address: usize,
    load_end: usize,
    allocated_end: usize,
}

impl LoadedSegment {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn load_address(&self) -> usize {
        self.load_address
    }
    pub fn load_end(&self) -> usize {
        self.load_end
    }
    pub fn allocated_address(&self) -> usize {
        self.pages.start_address()
    }
    pub fn allocated_end(&self) -> usize {
        self.allocated_end
    }
    pub fn allocated_pages(&self) -> usize {
        self.pages.page_count()
    }
    pub fn mem_size(&self) -> u64 {
        self.header.p_memsz
    }
    pub fn memory_size(&self) -> usize {
        self.load_end - self.load_address
    }
    pub fn file_offset(&self) -> usize {
        self.header.p_offset as usize
    }
    pub fn file_size(&self) -> usize {
        self.header.p_filesz as usize
    }
    pub fn file_end(&self) -> usize {
        self.file_offset() + self.file_size()
    }
    pub fn flags(&self) -> ProgramHeaderFlags {
        self.header.p_flags
    }
}

pub fn load_kernel_segments(
    bytes: &[u8],
    header: &Elf64Ehdr,
) -> Result<LoadedKernel, ElfLoadError> {
    let mut segments = Vec::new();

    for (index, program_header) in ProgramHeaders::new(bytes, header)?.enumerate() {
        let program_header = program_header?;

        if program_header.p_type == ProgramHeaderType::Load {
            segments.push(load_segment(bytes, index, program_header)?);
        }
    }

    if segments.is_empty() {
        info!("No PT_LOAD program headers found");
    }

    Ok(LoadedKernel {
        entry: header.e_entry,
        segments,
    })
}

fn load_segment(
    bytes: &[u8],
    index: usize,
    program_header: Elf64Phdr,
) -> Result<LoadedSegment, ElfLoadError> {
    if program_header.p_filesz > program_header.p_memsz {
        return Err(ElfLoadError::SegmentFileLargerThanMemory);
    }

    let file_offset =
        usize::try_from(program_header.p_offset).map_err(|_| ElfLoadError::SegmentSizeOverflow)?;
    let file_size =
        usize::try_from(program_header.p_filesz).map_err(|_| ElfLoadError::SegmentSizeOverflow)?;
    let memory_size =
        usize::try_from(program_header.p_memsz).map_err(|_| ElfLoadError::SegmentSizeOverflow)?;
    let load_address = usize::try_from(program_header.p_paddr)
        .map_err(|_| ElfLoadError::SegmentAddressOverflow)?;
    let load_end = load_address
        .checked_add(memory_size)
        .ok_or(ElfLoadError::SegmentAddressOverflow)?;
    let file_end = file_offset
        .checked_add(file_size)
        .ok_or(ElfLoadError::SegmentFileRangeOutOfBounds)?;

    if file_end > bytes.len() {
        return Err(ElfLoadError::SegmentFileRangeOutOfBounds);
    }

    let allocation_address = align_down_to_page(load_address);
    let allocation_offset = page_offset(load_address);
    let allocation_size = allocation_offset
        .checked_add(memory_size)
        .ok_or(ElfLoadError::SegmentSizeOverflow)?;
    let allocated_end = allocation_address
        .checked_add(align_up_to_page(allocation_size)?)
        .ok_or(ElfLoadError::SegmentAddressOverflow)?;
    let pages = LoadedPages::allocate_at(allocation_address, allocation_size)?;

    unsafe {
        let destination = load_address as *mut u8;
        ptr::copy_nonoverlapping(bytes.as_ptr().add(file_offset), destination, file_size);
        ptr::write_bytes(destination.add(file_size), 0, memory_size - file_size);
    }

    Ok(LoadedSegment {
        index,
        header: program_header,
        pages,
        load_address,
        load_end,
        allocated_end,
    })
}

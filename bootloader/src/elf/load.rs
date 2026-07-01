use alloc::vec::Vec;
use core::ptr;

use log::info;

use crate::elf::{
    constants::{Elf64Ehdr, Elf64Phdr, ElfLoadError, ProgramHeaderFlags, ProgramHeaderType},
    pages::{align_down_to_page, align_up_to_page, page_offset, LoadedPages},
    program_header::ProgramHeaders,
};

/// ## Loaded ELF kernel and its segments.
pub struct LoadedKernel {
    /// ELF entry address.
    entry: u64,
    /// Loaded segments.
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

/// ## Loaded ELF segment.
pub struct LoadedSegment {
    /// Index of the segment in the program header table.
    index: usize,
    /// ELF program header for the segment.
    header: Elf64Phdr,
    /// Allocated pages for the segment.
    pages: LoadedPages,
    /// Load address of the segment in memory.
    load_address: usize,
    /// End address of the segment in memory.
    load_end: usize,
    /// End address of the allocated pages for the segment.
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

/// ## Loads the kernel segments from the ELF file bytes and returns `LoadedKernel`.
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

/// ## Loads one `PT_LOAD` segment from the ELF file bytes
/// Copies the segments file-backed bytes into memory, then zero-initilaises the
/// remaining memory range used for `.bss` and other uninitialised data.
/// 
/// Returns `LoadedSegment` containing the segments load address, size and allocated pages.
/// 
/// ## Errors
///  - the segment declares more file data than memory size;
///  - an ELF address, offset, or size cannot fit in `usize`;
///  - an address or size calculation overflow;
///  - the segment file range lies outside `bytes`;
///  - UEFI cannot allocate required pages.
fn load_segment(
    bytes: &[u8],
    index: usize,
    program_header: Elf64Phdr,
) -> Result<LoadedSegment, ElfLoadError> {
    // Loadable segment cannot require more file data than its in-memory size.
    if program_header.p_filesz > program_header.p_memsz {
        return Err(ElfLoadError::SegmentFileLargerThanMemory);
    }

    // Convert ELF fields into native address-space values before pointer arithmetic.
    let file_offset =
        usize::try_from(program_header.p_offset).map_err(|_| ElfLoadError::SegmentSizeOverflow)?;
    let file_size =
        usize::try_from(program_header.p_filesz).map_err(|_| ElfLoadError::SegmentSizeOverflow)?;
    let memory_size =
        usize::try_from(program_header.p_memsz).map_err(|_| ElfLoadError::SegmentSizeOverflow)?;
    let load_address = usize::try_from(program_header.p_paddr)
        .map_err(|_| ElfLoadError::SegmentAddressOverflow)?;
    
    // Calculate end addresses while detecting integer overflow/
    let load_end = load_address
        .checked_add(memory_size)
        .ok_or(ElfLoadError::SegmentAddressOverflow)?;
    let file_end = file_offset
        .checked_add(file_size)
        .ok_or(ElfLoadError::SegmentFileRangeOutOfBounds)?;

    // Ensure the segment file range lies within the ELF file bytes.
    if file_end > bytes.len() {
        return Err(ElfLoadError::SegmentFileRangeOutOfBounds);
    }

    /*  
      UEFI allocates whole pages, so include the offset of the requested load address
      within the first page when calculating the allocation size.
    */
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

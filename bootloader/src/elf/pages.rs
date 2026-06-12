use core::ptr::NonNull;

use uefi::boot::{self, AllocateType, MemoryType, PAGE_SIZE};

use crate::elf::constants::ElfLoadError;

pub struct LoadedPages {
    ptr: NonNull<u8>,
    page_count: usize,
}

impl LoadedPages {
    pub fn allocate_at(address: usize, byte_len: usize) -> Result<Self, ElfLoadError> {
        if address == 0 {
            return Err(ElfLoadError::NullLoadAddress);
        }

        let page_count = page_count(byte_len)?;
        let ptr = boot::allocate_pages(
            AllocateType::Address(address as u64),
            MemoryType::LOADER_DATA,
            page_count,
        )
        .map_err(|err| ElfLoadError::AllocatePages(err.status()))?;

        Ok(Self { ptr, page_count })
    }

    pub fn start_address(&self) -> usize {
        self.ptr.as_ptr() as usize
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }
}

impl Drop for LoadedPages {
    fn drop(&mut self) {
        let _ = unsafe { boot::free_pages(self.ptr, self.page_count) };
    }
}

pub fn align_down_to_page(address: usize) -> usize {
    address & !(PAGE_SIZE - 1)
}

pub fn page_offset(address: usize) -> usize {
    address & (PAGE_SIZE - 1)
}

pub fn page_count(byte_len: usize) -> Result<usize, ElfLoadError> {
    Ok(align_up_to_page(byte_len)? / PAGE_SIZE)
}

pub fn align_up_to_page(byte_len: usize) -> Result<usize, ElfLoadError> {
    byte_len
        .checked_add(PAGE_SIZE - 1)
        .map(|len| len & !(PAGE_SIZE - 1))
        .ok_or(ElfLoadError::SegmentSizeOverflow)
}

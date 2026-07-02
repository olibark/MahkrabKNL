//! # Page Management for ELF loading.
//!
//! Provides helpers for converting byte ranges into UEFI page
//! allocations and a small RAII wrapper that frees those allocations when
//! dropped.

use core::ptr::NonNull;

use uefi::boot::{self, AllocateType, MemoryType, PAGE_SIZE};

use crate::elf::constants::ElfLoadError;

/// ### Page-alined UEFI allocation used for a loaded kernel segment.
/// Allocation is released automatically when this value is dropped.
///
/// It represents whole UEFI pages rather than an arbitrary byte-length range.
pub struct LoadedPages {
    /// Pointer to first byte of the allocated page range.
    ptr: NonNull<u8>,
    /// Number of UEFI pages owned by this allocation.
    page_count: usize,
}

impl LoadedPages {
    /// ### Allocates enough UEFI pages to hold `byte_len` bytes at `address`
    /// Allocation uses [`MemoryType::LOADER_DATA`] as it is owned by
    /// the bootloader while loading the kernel.
    ///
    /// ### Errors
    ///  - `address` is zero;
    ///  - rounding `byte_len` up t a page boundry overflows.;
    ///  - UEFI cannot allocate pages at te requested address.
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

    /// Returns the physical address of the first allocated page.
    #[must_use]
    pub fn start_address(&self) -> usize {
        self.ptr.as_ptr() as usize
    }
    /// Returns the number of UEFI pages in this allocation.
    #[must_use]
    pub fn page_count(&self) -> usize {
        self.page_count
    }
}

impl Drop for LoadedPages {
    /// ### Releases the UEFI page allocation.
    /// Errors cannot be returned from [`Drop`], so failed declaration
    /// is ignored. This shoudl only run while UEFI boot services remain
    /// unavaliable.
    fn drop(&mut self) {
        let _ = unsafe { boot::free_pages(self.ptr, self.page_count) };
    }
}

/// ### Rounds `address` down to the start of its containing UEFI page.
#[must_use]
pub fn align_down_to_page(address: usize) -> usize {
    address & !(PAGE_SIZE - 1)
}

/// Returns the offset of `address` within its containing UEFI page.
#[must_use]
pub fn page_offset(address: usize) -> usize {
    address & (PAGE_SIZE - 1)
}

/// ### Returns the number of UEFI pages required to store `byte_len` bytes.
///
/// ### Errors:
///  - If rounding byte length would overflow `usize`: returns [`ElfLoadError::SegmentSizeOverflow`]
pub fn page_count(byte_len: usize) -> Result<usize, ElfLoadError> {
    Ok(align_up_to_page(byte_len)? / PAGE_SIZE)
}

/// ### Rounds `byte_len` up to the next multiple of [`PAGE_SIZE`].
/// Retuurns `byte_len` unchanged when it is already page-aligned.
///
/// ### Errors:
///  - If rounding byte lenght would overflow `usize`: returns [`ElfLoadError::SegmentSizeOverflow`].
pub fn align_up_to_page(byte_len: usize) -> Result<usize, ElfLoadError> {
    byte_len
        .checked_add(PAGE_SIZE - 1)
        .map(|len| len & !(PAGE_SIZE - 1))
        .ok_or(ElfLoadError::SegmentSizeOverflow)
}

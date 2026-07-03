use core::mem;

use bootprotocol::BootInfo;

use crate::elf::load::{LoadedKernel, LoadedSegment};

/// ### Kernel entry-point function type
/// Transfers execution to loaded kernel.
///
/// Uses the System V AMD64 calling convension and never returns.
pub type KernelEntry = unsafe extern "sysv64" fn(*const BootInfo) -> !;

/// Errors returned while validating ELF kernel entry point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryPointError {
    /// ELF entry address does not fit in usize.
    AddressOverflow,
    /// The entry address is not in an executable load segment.
    NotInExecutableLoadSegment,
}

/// ### Validated kernel entry point
/// Confirms the ELF entry address fits in `usize`` and lies within
/// an executable load segment.
///
/// Does not prove validity of machine instructions.
#[derive(Clone, Copy)]
pub struct ValidatedKernelEntry {
    address: usize,
    segment_index: usize,
    function: KernelEntry,
}

impl ValidatedKernelEntry {
    /// Returns the validated memory address of kernel entry point.
    pub fn address(&self) -> usize {
        self.address
    }
    /// Returns the index of the load segment containing the kernel entry point.
    pub fn segment_index(&self) -> usize {
        self.segment_index
    }
    /// Returns the validated kernel entry point as a callable function pointer.
    pub fn function(&self) -> KernelEntry {
        self.function
    }
}

/// ### Validates the entry point declared in the loaded ELF kernel.
/// Entry must fit in `usize and be in an executable loaded
/// segment before it is converted to [`KernelEntry`] function pointer.
pub fn validate_kernel_entry(
    loaded_kernel: &LoadedKernel,
) -> Result<ValidatedKernelEntry, EntryPointError> {
    let entry_address =
        usize::try_from(loaded_kernel.entry()).map_err(|_| EntryPointError::AddressOverflow)?;

    let entry_segment = loaded_kernel
        .segments()
        .iter()
        .find(|segment| contains_executable_entry(segment, entry_address))
        .ok_or(EntryPointError::NotInExecutableLoadSegment)?;

    /*
      Address is valid and in an executable segment. The kernel ELF is trusted to
      provide a valid entry point using System V AMD64 convention.
    */
    let function = unsafe { mem::transmute::<usize, KernelEntry>(entry_address) };

    Ok(ValidatedKernelEntry {
        address: entry_address,
        segment_index: entry_segment.index(),
        function,
    })
}

/// ### Returns whether `entry_address` is in an executable load segment.
fn contains_executable_entry(segment: &LoadedSegment, entry_address: usize) -> bool {
    segment.flags().execute()
        && entry_address >= segment.load_address()
        && entry_address < segment.load_end()
}

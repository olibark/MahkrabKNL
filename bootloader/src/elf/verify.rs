//! # Provides functionality for veryfying the loaded kernel segments.

use core::{fmt, slice};

use crate::elf::load::{LoadedKernel, LoadedSegment};

/// ### Represents the status of a segments veryfication of the loaded kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    /// Segment verified.
    Ok,
    /// Segment verification failed.
    Failed,
    /// Segment verification not needed.
    NotNeeded,
}

impl fmt::Display for VerificationStatus {
    /// ### Formats tje [`VerificationStatus`] as a string.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Ok => "ok",
            Self::Failed => "failed",
            Self::NotNeeded => "not-needed",
        };

        formatter.write_str(text)
    }
}

/// ### Represents the recipt of a segment verification of the loaded kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentVerification {
    /// Index of the segment in the laoded kernel.
    index: usize,
    /// Start address of the segment in physical memory.
    memory_start: usize,
    /// End address of the segment in physcial memory.
    memory_end: usize,
    /// Status of the file copy verification.
    file_copy: VerificationStatus,
    /// Status of the zero fill verification.
    zero_fill: VerificationStatus,
}

impl SegmentVerification {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn memory_start(&self) -> usize {
        self.memory_start
    }
    pub fn memory_end(&self) -> usize {
        self.memory_end
    }
    pub fn file_copy(&self) -> VerificationStatus {
        self.file_copy
    }
    pub fn zero_fill(&self) -> VerificationStatus {
        self.zero_fill
    }
}

/// ### Iterator that verifies each segment of the loaded kernel.
pub struct SegmentVerifications<'a> {
    /// Kernel bytes used for verification.
    kernel_bytes: &'a [u8],
    /// Iterator over the loaded kernel segments.
    /// Yields a reference to a [`LoadedSegment`] on each iteration.
    segments: slice::Iter<'a, LoadedSegment>,
}

impl<'a> SegmentVerifications<'a> {
    /// Creates a new [`SegmentVerifications`] iterator.
    fn new(kernel_bytes: &'a [u8], loaded_kernel: &'a LoadedKernel) -> Self {
        Self {
            kernel_bytes,
            segments: loaded_kernel.segments().iter(),
        }
    }
}

impl Iterator for SegmentVerifications<'_> {
    /// The verification result prouduced for each segment.
    type Item = SegmentVerification;
    /// ### Advances the iterator, veryfying the next segment.
    /// Returns `Some(SegmentVerification)` if ther is a next segment
    /// or `None` if iterator is exhausted. *lazy*.
    fn next(&mut self) -> Option<Self::Item> {
        self.segments
            .next()
            .map(|segment| verify_segment(self.kernel_bytes, segment))
    }
}

/// ### Verifies the loaded kernel segments agains the kernel bytes.
/// Returns an iterator over the [`SegmentVerification`] for each segment.
pub fn verify_loaded_segments<'a>(
    kernel_bytes: &'a [u8],
    loaded_kernel: &'a LoadedKernel,
) -> SegmentVerifications<'a> {
    SegmentVerifications::new(kernel_bytes, loaded_kernel)
}

/// ### Compares a single loaded segment against its source bytes.
/// **Verifies:**
///  * The data copied from the file mathces the expected kernel bytes.
///  * Any extra padding memory (BSS section) is correctly zeroed.
fn verify_segment(kernel_bytes: &[u8], segment: &LoadedSegment) -> SegmentVerification {
    let file_size = segment.file_size();
    let memory_size = segment.memory_size();

    // Extract the sacred "Source of Truth" (spooky) slice from the raw kernel image.
    let source = &kernel_bytes[segment.file_offset()..segment.file_end()];
    /*
      SAFETY:
      Assumes the load address points to a valid, initilized memory region
      mapped by the system loader, and that file size accuratly represents its bounds.
    */
    let loaded_file =
        unsafe { slice::from_raw_parts(segment.load_address() as *const u8, file_size) };

    // Verify that the loaded program code matches the source bytes exeactly.
    let file_copy = if loaded_file == source {
        VerificationStatus::Ok
    } else {
        VerificationStatus::Failed
    };

    /*
      If the segment has a larger memory size than file size, verify that tht extra memory
      is zeroed. This is hte BSS section, which is expected to be zeroed by the loader.
    */
    let zero_fill = if memory_size > file_size {
        let zero_fill_start = segment.load_address() + file_size;
        let zero_fill_size = memory_size - file_size;

        /*
          SAFETY:
          Assumes the memory range allocated for zeroing is valid
          and accessible up to memory size.
        */
        let zero_fill =
            unsafe { slice::from_raw_parts(zero_fill_start as *const u8, zero_fill_size) };

        if zero_fill.iter().all(|byte| *byte == 0) {
            VerificationStatus::Ok
        } else {
            VerificationStatus::Failed
        }
    } else {
        VerificationStatus::NotNeeded
    };

    // Return the verification result for this segment.
    SegmentVerification {
        index: segment.index(),
        memory_start: segment.load_address(),
        memory_end: segment.load_end(),
        file_copy,
        zero_fill,
    }
}

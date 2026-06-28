use core::{fmt, slice};

use crate::elf::load::{LoadedKernel, LoadedSegment};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    Ok,
    Failed,
    NotNeeded,
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Ok => "ok",
            Self::Failed => "failed",
            Self::NotNeeded => "not-needed",
        };

        formatter.write_str(text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentVerification {
    index: usize,
    memory_start: usize,
    memory_end: usize,
    file_copy: VerificationStatus,
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

pub struct SegmentVerifications<'a> {
    kernel_bytes: &'a [u8],
    segments: slice::Iter<'a, LoadedSegment>,
}

impl<'a> SegmentVerifications<'a> {
    fn new(kernel_bytes: &'a [u8], loaded_kernel: &'a LoadedKernel) -> Self {
        Self {
            kernel_bytes,
            segments: loaded_kernel.segments().iter(),
        }
    }
}

impl Iterator for SegmentVerifications<'_> {
    type Item = SegmentVerification;

    fn next(&mut self) -> Option<Self::Item> {
        self.segments
            .next()
            .map(|segment| verify_segment(self.kernel_bytes, segment))
    }
}

pub fn verify_loaded_segments<'a>(
    kernel_bytes: &'a [u8],
    loaded_kernel: &'a LoadedKernel,
) -> SegmentVerifications<'a> {
    SegmentVerifications::new(kernel_bytes, loaded_kernel)
}

fn verify_segment(kernel_bytes: &[u8], segment: &LoadedSegment) -> SegmentVerification {
    let file_size = segment.file_size();
    let memory_size = segment.memory_size();
    let source = &kernel_bytes[segment.file_offset()..segment.file_end()];

    let loaded_file =
        unsafe { slice::from_raw_parts(segment.load_address() as *const u8, file_size) };
    let file_copy = if loaded_file == source {
        VerificationStatus::Ok
    } else {
        VerificationStatus::Failed
    };

    let zero_fill = if memory_size > file_size {
        let zero_fill_start = segment.load_address() + file_size;
        let zero_fill_size = memory_size - file_size;
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

    SegmentVerification {
        index: segment.index(),
        memory_start: segment.load_address(),
        memory_end: segment.load_end(),
        file_copy,
        zero_fill,
    }
}

#![no_std]

/// ### Boot information passed from the bootloader to the kernel.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BootInfo {
    /// Framebuffer base physical address.
    pub framebuffer_base: u64,
    /// Framebuffer size in bytes.
    pub framebuffer_size: u64,
    /// Horizontal resolution in pixels.
    pub width: u32,
    /// Vertical resolution in pixels.
    pub height: u32,
    /// Pixels per framebuffer scan line.
    pub pixels_per_scan_line: u32,
    /// UEFI GOP pixel format value.
    pub pixel_format: u32,
}

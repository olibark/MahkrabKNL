//! # Boot information passed from the UEFI bootloader to the kernel.
//! The bootloader gathers display information from UEFI graphics output
//! (GOP), converts into shared [`BootInfo`] ABI, and stores in
//! memory that remains valid after control transfers to the kernel. 
//! lucky memory.
//! 
//! [`build_boot_info`] must be called exactly once during boot, before kernel
//! is entered.

use core::{cell::UnsafeCell, fmt, mem::MaybeUninit};

use bootprotocol::BootInfo;
use uefi::{
    boot,
    proto::console::gop::{GraphicsOutput, PixelFormat},
};

/// ### Errors returned while building kernel boot information.
#[derive(Debug)]
pub enum BootInfoError {
    /// GOP handle non existant.
    GopHandle(uefi::Error),
    /// GOP could not be opened.
    GopOpen(uefi::Error),
    /// GOP field does not fit in the boot protocol.
    FieldOverflow,
    /// GOP mode has no CPU-visible framebuffer.
    BltOnlyFramebuffer,
}

impl fmt::Display for BootInfoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BootInfoError::GopHandle(err) => write!(formatter, "could not find GOP handle: {err}"),
            BootInfoError::GopOpen(err) => write!(formatter, "could not open GOP: {err}"),
            BootInfoError::FieldOverflow => write!(formatter, "GOP field does not fit BootInfo"),
            BootInfoError::BltOnlyFramebuffer => {
                write!(formatter, "GOP mode does not expose a direct framebuffer")
            }
        }
    }
}

/// ### Storage for the single [`BootInfo`] instance passed to the kernel.
/// The value begins uninitilaised, it is written once by [`build_boot_info`], and
/// is treated as immutable for the remainder of the boot process.
struct BootInfoSlot(UnsafeCell<MaybeUninit<BootInfo>>);

/// [`BootInfo`] is written once before it is read by the kernel.
unsafe impl Sync for BootInfoSlot {}

/// Static storage whose address remains valid after the bootloader jumps to the kernel
/// entry point.
static BOOT_INFO: BootInfoSlot = BootInfoSlot(UnsafeCell::new(MaybeUninit::uninit()));

/// ### Builds and stores the display related boot info for the kernel.
/// This reads the active UEFI GOP mode and records the framebuffer address, size, resolution,
/// scan stride, and format in a shared [`BootInfo`] struct.
/// 
/// The returned reference remains calid for the rest of boot and afeter the kernel hand off.
/// 
/// ### Errors:
///  - No GOP handle is avaliable;
///  - The protocol cannot be opened;
///  - The active mode uses [`PixelFormat::BltOnly`] rather than a direct framebfuffer;
///  - A GOP value does not fit in the [`BootInfo`] field. *Big cow*
/// 
/// This function must be called once. Reinitiliasing [`BootInfo`] after a reference
/// has been handed to the kernel would **blow up the world**.
pub fn build_boot_info() -> Result<&'static BootInfo, BootInfoError> {
    let handle =
        boot::get_handle_for_protocol::<GraphicsOutput>().map_err(BootInfoError::GopHandle)?;
    let mut gop =
        boot::open_protocol_exclusive::<GraphicsOutput>(handle).map_err(BootInfoError::GopOpen)?;

    let mode_info = gop.current_mode_info();
    let (width, height) = mode_info.resolution();
    let pixels_per_scan_line = mode_info.stride();
    let pixel_format = mode_info.pixel_format();

    if pixel_format == PixelFormat::BltOnly {
        return Err(BootInfoError::BltOnlyFramebuffer);
    }

    let pixel_format = pixel_format_code(pixel_format);
    let mut framebuffer = gop.frame_buffer();

    let boot_info = BootInfo {
        framebuffer_base: framebuffer.as_mut_ptr() as u64,
        framebuffer_size: u64::try_from(framebuffer.size())
            .map_err(|_| BootInfoError::FieldOverflow)?,
        width: u32::try_from(width).map_err(|_| BootInfoError::FieldOverflow)?,
        height: u32::try_from(height).map_err(|_| BootInfoError::FieldOverflow)?,
        pixels_per_scan_line: u32::try_from(pixels_per_scan_line)
            .map_err(|_| BootInfoError::FieldOverflow)?,
        pixel_format,
    };

    Ok(store_boot_info(boot_info))
}

/// ### Stores [`BootInfo`] in static memory used for the kernel hand off.
/// This function must be called only once. After the retyurned reference escapes,
/// the memory must never be written again. *No one can speak of this memory again*.
fn store_boot_info(boot_info: BootInfo) -> &'static BootInfo {
    unsafe { (*BOOT_INFO.0.get()).write(boot_info) }
}

/// ### Converts GOP pixel format into the shared ABI value.
/// This assumes that [`bootprotocol::BootInfo::pixel_format`] intentionally uses
/// the same discriminants (*i hate maths*) as UEFI [`PixelFormat`] enum.
#[must_use]
fn pixel_format_code(pixel_format: PixelFormat) -> u32 {
    pixel_format as u32
}

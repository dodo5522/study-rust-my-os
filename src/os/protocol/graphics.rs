use core::mem::offset_of;
use core::ptr::null_mut;
use crate::os::EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
use crate::os::{EfiStatus, EfiSystemTable, EfiVoid, Result};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EfiGraphicsOutputProtocolPixelInfo {
    version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    _padding: [u32; 5],
    pub pixels_per_scan_line: u32,
}
const _: () = assert!(offset_of!(EfiGraphicsOutputProtocolPixelInfo, pixels_per_scan_line) == 32);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mode: u32,
    pub mode: u32,
    pub info: &'a EfiGraphicsOutputProtocolPixelInfo,
    pub size_of_info: u64,
    pub frame_buffer_base: usize,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EfiGraphicsOutputProtocol<'a> {
    reserved: [u64; 3],
    pub mode: &'a EfiGraphicsOutputProtocolMode<'a>,
}

pub fn locate_graphics_protocol<'a>(efi_system_table: &EfiSystemTable) -> Result<&'a EfiGraphicsOutputProtocol<'a>> {
    let mut graphic_output_protocol = null_mut::<EfiGraphicsOutputProtocol>();
    let status = (efi_system_table.boot_services.locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        null_mut::<EfiVoid>(),
        &mut graphic_output_protocol as *mut *mut EfiGraphicsOutputProtocol as *mut *mut EfiVoid,
    );
    if status != EfiStatus::Success {
        return Err("Failed to locate graphics output protocol.");
    }
    Ok(unsafe {&*graphic_output_protocol})
}

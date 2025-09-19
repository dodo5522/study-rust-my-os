#![no_std]
#![no_main]
#![feature(offset_of)]

mod os;
mod handler;

use core::mem::size_of;
use core::slice;
use os::{EfiHandle, EfiSystemTable, locate_graphics_protocol};

#[no_mangle]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let efi_graphics_output_protocol = locate_graphics_protocol(efi_system_table).expect("Cannot locate graphics protocol");
    let vram_addr = efi_graphics_output_protocol.mode.frame_buffer_base;
    let vram_size = efi_graphics_output_protocol.mode.frame_buffer_size;
    let vram = unsafe {
        slice::from_raw_parts_mut(vram_addr as *mut u32, vram_size / size_of::<u32>())
    };

    for e in vram {
        *e = 0xffffff; // white
    }

    loop {}
}

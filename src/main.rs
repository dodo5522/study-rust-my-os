#![no_std]
#![no_main]
#![feature(offset_of)]

mod os;
mod handler;

use os::{EfiHandle, EfiSystemTable};

#[no_mangle]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    // println!("Hello, world!");
    loop {}
}

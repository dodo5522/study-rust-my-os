use core::panic::PanicInfo;

use super::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {
    asm::halt();
  }
}

#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use vga_buffer::{Writer, ColorCode, Color};


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


static HELLO: &[u8] = b":-)";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();
    
    writer.color_code = ColorCode::new(Color::Red, Color::Black);
    writer.write_str(":");

    writer.color_code = ColorCode::new(Color::Yellow, Color::Black);
    writer.write_str("-");

    writer.color_code = ColorCode::new(Color::Cyan, Color::Black);
    writer.write_str(")");




    
    loop {}
}

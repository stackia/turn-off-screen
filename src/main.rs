#![no_main]
#![no_std]

use windows_sys::Win32::UI::WindowsAndMessaging;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    unsafe {
        WindowsAndMessaging::PostMessageW(
            0xffff,
            WindowsAndMessaging::WM_SYSCOMMAND,
            usize::try_from(WindowsAndMessaging::SC_MONITORPOWER).unwrap(),
            2,
        );
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

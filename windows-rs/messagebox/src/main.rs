use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("Hello"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("Hello"), MB_OK);
        MessageBoxW(None, h!("WinRT"), h!("Hello"), MB_OK);
    }
}

#![windows_subsystem = "windows"]

use windows_sys::Win32::{System::Power::SetSuspendState};

fn main() {
    unsafe {
        SetSuspendState(false.into(), true.into(), false.into());
    }
}

#![no_std]

extern crate alloc;
pub mod io;
pub mod error;

use alloc::ffi::CString;
use wdk_sys::ntddk::DbgPrint;

use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING};

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

#[export_name = "DriverEntry"]
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    welcome_msg();

    driver.DriverUnload = Some(driver_exit);
    
    0
}

extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("[i] Sanctum driver cleaning up...");
}

fn welcome_msg() {
    let msg = "[i] Starting Sanctum driver...";
    println!("{}", msg);
}
use wdk_sys::ntddk::DbgPrint;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let s = alloc::format!("[sanctum-driver]: {}\0", args);

    // print the string
    unsafe { DbgPrint(s.as_ptr() as _) };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        ($crate::io::_print(format_args!($($arg)*)));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
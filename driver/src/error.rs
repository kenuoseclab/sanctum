use wdk_sys::{NTSTATUS, STATUS_NOT_IMPLEMENTED, STATUS_UNSUCCESSFUL};

pub struct Error(NTSTATUS);

/// With the Error type defined, we can now simply use Result<T, Error> where Ok replaces STATUS_SUCCESS.
impl Error {
    // https://www.synkhronix.com/guides/windows-dev/windows-drivers-in-rust-safe-framework/
    pub const UNSUCCESSFUL: Error = Error(STATUS_UNSUCCESSFUL);
    pub const NOT_IMPLEMENTED: Error = Error(STATUS_NOT_IMPLEMENTED);

    pub fn from_kernel_errno(status: NTSTATUS) -> Error {
        Error(status)
    }

    pub fn to_kernel_errno(&self) -> NTSTATUS {
        self.0
    }
}


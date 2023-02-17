#![no_std]

mod helper;
mod host;
mod raw;

use alloc::boxed::Box;
pub use host::*;
pub use raw::*;

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum Error {
    None,
    OutOfMemory,
    TypeMismatch,
    NoSuchNode,
    OutOfBounds,
    ExecutionFailure,
    IllegalArguments,

    /// Evaluating external inputs (e.g., nodes of the ACPI namespace) returned an unexpected result.
    /// Unlike LAI_ERROR_EXECUTION_FAILURE, this error does not indicate that
    /// execution of AML failed; instead, the resulting object fails to satisfy some
    /// expectation (e.g., it is of the wrong type, has an unexpected size, or consists of
    /// unexpected contents)
    UnexpectedResult,
    /// Error given when end of iterator is reached, nothing to worry about.
    EndReached,
    NotSupported,
}

extern crate alloc;

/// Initializes the ACPI revision.
#[inline]
pub fn set_acpi_revision(revision: i32) {
    unsafe { raw::lai_set_acpi_revision(revision) }
}

/// Creates the ACPI namespace.
#[inline]
pub fn create_namespace() {
    unsafe { raw::lai_create_namespace() }
}

/// Enables ACPI SCI.
///
/// ## Parameters
/// * `mode`: IRQ mode (ACPI spec section 5.8.1)
pub fn enable_acpi(mode: u32) {
    unsafe { assert_eq!(raw::lai_enable_acpi(mode), raw::LAI_SUCCESS) }
}

/// Enters a sleeping state.
///
/// ## Parameters
/// * `sleep_state`: 0-5 to correspond with states S0-S5
pub fn enter_sleep(sleep_state: u8) {
    unsafe { assert_eq!(raw::lai_enter_sleep(sleep_state), raw::LAI_SUCCESS) }
}

pub fn reset() {
    unsafe { assert_eq!(raw::lai_acpi_reset(), raw::LAI_SUCCESS) }
}

pub fn pci_route_pin(
    seg: u16,
    bus: u8,
    slot: u8,
    function: u8,
    pin: u8,
) -> Result<Box<AcpiResource>, Error> {
    let mut dest = Box::new(AcpiResource::default());
    unsafe {
        let result = raw::lai_pci_route_pin(
            &mut *dest as *mut AcpiResource,
            seg,
            bus,
            slot,
            function,
            pin,
        );

        if result != LAI_SUCCESS {
            let err: Error = core::mem::transmute(result);
            Err(err)
        } else {
            Ok(dest)
        }
    }
}

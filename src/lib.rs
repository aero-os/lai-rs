#![no_std]

mod helper;
mod host;
mod raw;

pub use host::*;

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

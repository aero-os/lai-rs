#![no_std]

extern crate alloc;

mod helper;
mod host;
mod sys;

use core::mem::MaybeUninit;
pub use host::*;
pub use sys::*;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Error {
    OutOfMemory = lai_api_error_LAI_ERROR_OUT_OF_MEMORY,
    TypeMismatch = lai_api_error_LAI_ERROR_TYPE_MISMATCH,
    NoSuchNode = lai_api_error_LAI_ERROR_NO_SUCH_NODE,
    OutOfBounds = lai_api_error_LAI_ERROR_OUT_OF_BOUNDS,
    ExecutionFailure = lai_api_error_LAI_ERROR_EXECUTION_FAILURE,
    IllegalArguments = lai_api_error_LAI_ERROR_ILLEGAL_ARGUMENTS,

    /// Evaluating external inputs (e.g., nodes of the ACPI namespace) returned an unexpected result.
    /// Unlike LAI_ERROR_EXECUTION_FAILURE, this error does not indicate that
    /// execution of AML failed; instead, the resulting object fails to satisfy some
    /// expectation (e.g., it is of the wrong type, has an unexpected size, or consists of
    /// unexpected contents)
    UnexpectedResult = lai_api_error_LAI_ERROR_UNEXPECTED_RESULT,
    /// Error given when end of iterator is reached, nothing to worry about.
    EndReached = lai_api_error_LAI_ERROR_END_REACHED,
    Unsupported = lai_api_error_LAI_ERROR_UNSUPPORTED,
}

fn lai_call(e: lai_api_error) -> Result<(), Error> {
    if e == lai_api_error_LAI_ERROR_NONE {
        Ok(())
    } else {
        Err(unsafe { core::mem::transmute(e) })
    }
}

/// Initializes the ACPI revision.
#[inline]
pub fn set_acpi_revision(revision: i32) {
    unsafe { lai_set_acpi_revision(revision) }
}

/// Creates the ACPI namespace.
#[inline]
pub fn create_namespace() {
    unsafe { lai_create_namespace() }
}

#[repr(u32)]
pub enum PICMethod {
    PIC = 0,
    APIC = 1,
    SAPIC = 2,
}

/// Enables ACPI SCI.
///
/// ## Parameters
/// * `mode`: IRQ mode (ACPI spec section 5.8.1)
pub fn enable_acpi(mode: PICMethod) {
    // lai_enable_acpi is infallible, and does not return lai_api_error_t
    unsafe { assert_eq!(lai_enable_acpi(mode as _), 0) }
}

#[repr(u8)]
pub enum SleepState {
    Normal = 0,
    Standby = 1,
    SuspendToRam = 3,
    SuspendToDisk = 4,
    Shutdown = 5,
}

/// Enters a sleeping state.
///
/// ## Parameters
/// * `sleep_state`: 0-5 to correspond with states S0-S5
pub fn enter_sleep(sleep_state: SleepState) -> Result<(), Error> {
    lai_call(unsafe { lai_enter_sleep(sleep_state as _) })
}

pub fn reset() -> Result<(), Error> {
    lai_call(unsafe { lai_acpi_reset() })
}

pub fn pci_route_pin(
    seg: u16,
    bus: u8,
    slot: u8,
    function: u8,
    pin: u8,
) -> Result<acpi_resource_t, Error> {
    let mut dest = MaybeUninit::uninit();
    unsafe {
        lai_call(lai_pci_route_pin(
            dest.as_mut_ptr(),
            seg,
            bus,
            slot,
            function,
            pin,
        ))?;

        Ok(dest.assume_init())
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct SciEvent: u16 {
        const TIMER        = 0x0001;
        const BUSMASTER    = 0x0010;
        const GLOBAL       = 0x0020;
        const POWER_BUTTON = 0x0100;
        const SLEEP_BUTTON = 0x0200;
        const RTC_ALARM    = 0x0400;
        const PCIE_WAKE    = 0x4000;
        const WAKE         = 0x8000;
    }
}

pub fn get_sci_event() -> SciEvent {
    SciEvent::from_bits_retain(unsafe { lai_get_sci_event() })
}

pub(crate) const LAI_SUCCESS: i32 = 0;

extern "C" {
    pub(crate) fn lai_set_acpi_revision(revison: i32);
    pub(crate) fn lai_create_namespace();
    pub(crate) fn lai_enable_acpi(mode: u32) -> i32;
    pub(crate) fn lai_enter_sleep(sleep_state: u8) -> i32;
}

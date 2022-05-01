/// Creates a Rust string from the provided C string.
///
/// ## Safety
/// - The provided pointer must be valid.
/// - The provided pointer must point to a null-terminated C string.
/// - The returned lifetime is not guaranteed to be the actual lifetime
/// of `ptr`.
/// - It is not guaranteed that the memory pointed by `ptr` won’t change
/// before the Rust string has been destroyed.
pub(crate) unsafe fn c_str_as_str<'cstring>(ptr: *const u8) -> &'cstring str {
    let length = c_strlen(ptr);
    let slice = core::slice::from_raw_parts(ptr, length);

    core::str::from_utf8_unchecked(slice)
}

/// Determines the string length of the given C string.
///
/// ## Safety
/// - The provided pointer must be valid.
/// - The provided pointer must point to a null-terminated C string.
pub(crate) unsafe fn c_strlen(mut ptr: *const u8) -> usize {
    let mut length = 0;

    while *ptr != 0 {
        ptr = ptr.offset(1);
        length += 1;
    }

    length
}

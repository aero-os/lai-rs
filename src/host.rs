use core::alloc::Layout;

use alloc::boxed::Box;

use super::helper::*;

static mut LAI_HOST: Option<Box<dyn Host>> = None;

pub trait Host {
    fn scan(&self, _signature: &str, _index: usize) -> *const u8 {
        unimplemented!()
    }

    unsafe fn alloc(&self, size: usize) -> *mut u8 {
        let layout = Layout::from_size_align_unchecked(size, 16);
        alloc::alloc::alloc_zeroed(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, size: usize) {
        let layout = Layout::from_size_align_unchecked(size, 16);
        alloc::alloc::dealloc(ptr, layout);
    }

    unsafe fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
        let layout = Layout::from_size_align_unchecked(old_size, 16);
        alloc::alloc::realloc(ptr, layout, new_size)
    }
}

pub fn init(host: Box<dyn Host>) {
    unsafe {
        assert!(LAI_HOST.is_none());
        LAI_HOST = Some(host);
    }
}

// lai/include/lai/host.h
const LAI_DEBUG_LOG: i32 = 1;
const LAI_WARN_LOG: i32 = 2;

#[no_mangle]
extern "C" fn laihost_log(level: i32, message: *const u8) {
    let message = unsafe { c_str_as_str(message) };

    match level {
        LAI_DEBUG_LOG => log::debug!("{message}"),
        LAI_WARN_LOG => log::warn!("{message}"),

        _ => unreachable!("undefined log level: (message={message}, level={level})"),
    };
}

#[no_mangle]
extern "C" fn laihost_panic(message: *const u8) -> ! {
    let message = unsafe { c_str_as_str(message) };
    panic!("{message}");
}

#[no_mangle]
unsafe extern "C" fn laihost_malloc(size: usize) -> *mut u8 {
    LAI_HOST
        .as_ref()
        .expect("lai: host not initialized")
        .alloc(size)
}

#[no_mangle]
unsafe extern "C" fn laihost_free(ptr: *mut u8, size: usize) {
    LAI_HOST
        .as_ref()
        .expect("lai: host not initialized")
        .dealloc(ptr, size)
}

#[no_mangle]
unsafe extern "C" fn laihost_realloc(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    LAI_HOST
        .as_ref()
        .expect("lai: host not initialized")
        .realloc(ptr, old_size, new_size)
}

#[no_mangle]
unsafe extern "C" fn laihost_scan(signature: *const u8, index: usize) -> *const u8 {
    let signature = c_str_as_str(signature);
    LAI_HOST
        .as_ref()
        .expect("lai: host not initialized")
        .scan(signature, index)
}

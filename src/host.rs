use core::alloc::Layout;

use alloc::sync::Arc;

use super::helper::*;

static mut LAI_HOST: Option<Arc<dyn Host>> = None;

fn get_laihost() -> Arc<dyn Host> {
    unsafe {
        LAI_HOST
            .as_ref()
            .expect("lai: host not initialized")
            .clone()
    }
}

macro_rules! marker {
    ($(fn $name:tt(&self, $($pname:tt: $ptyp:ty),*) -> $ret:ty);*;) => {
        $(fn $name(&self, $($pname: $ptyp),*) -> $ret { unimplemented!() })*
    };
}

pub trait Host {
    marker!(
        fn scan(&self, _signature: &str, _index: usize) -> *const u8;

        // Port I/O functions:
        fn outb(&self, _port: u16, _value: u8) -> ();
        fn outw(&self, _port: u16, _value: u16) -> ();
        fn outd(&self, _port: u16, _value: u32) -> ();

        fn inb(&self, _port: u16) -> u8;
        fn inw(&self, _port: u16) -> u16;
        fn ind(&self, _port: u16) -> u32;
    );

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

pub fn init(host: Arc<dyn Host>) {
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
    get_laihost().alloc(size)
}

#[no_mangle]
unsafe extern "C" fn laihost_free(ptr: *mut u8, size: usize) {
    get_laihost().dealloc(ptr, size)
}

#[no_mangle]
unsafe extern "C" fn laihost_realloc(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    get_laihost().realloc(ptr, old_size, new_size)
}

#[no_mangle]
unsafe extern "C" fn laihost_scan(signature: *const u8, index: usize) -> *const u8 {
    let signature = c_str_as_str(signature);
    get_laihost().scan(signature, index)
}

// Port I/O functions:
#[no_mangle]
unsafe extern "C" fn laihost_outb(port: u16, value: u8) {
    get_laihost().outb(port, value)
}

#[no_mangle]
unsafe extern "C" fn laihost_outw(port: u16, value: u16) {
    get_laihost().outw(port, value)
}

#[no_mangle]
unsafe extern "C" fn laihost_outd(port: u16, value: u32) {
    get_laihost().outd(port, value)
}

#[no_mangle]
unsafe extern "C" fn laihost_inb(port: u16) -> u8 {
    get_laihost().inb(port)
}

#[no_mangle]
unsafe extern "C" fn laihost_inw(port: u16) -> u16 {
    get_laihost().inw(port)
}

#[no_mangle]
unsafe extern "C" fn laihost_ind(port: u16) -> u32 {
    get_laihost().ind(port)
}

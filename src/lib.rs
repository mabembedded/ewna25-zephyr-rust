#![no_std]

use core::ffi::{c_int, c_long};

extern "C" {
    fn ble_init() -> c_int;
    fn ble_start_advertising() -> c_int;
    fn ble_sleep_ms(milliseconds: c_long);  // Add this
}

#[no_mangle]
extern "C" fn rust_ble_connected() {
    zephyr::printkln!("Rust: BLE connected!");
}

#[no_mangle]
extern "C" fn rust_ble_disconnected() {
    zephyr::printkln!("Rust: BLE disconnected!");
}

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }
    
    zephyr::printkln!("Initializing BLE from Rust");
    
    unsafe {
        let err = ble_init();
        if err != 0 {
            zephyr::printkln!("BLE init failed: {}", err);
            return;
        }
        
        ble_sleep_ms(100);  // Clean and simple!
        
        let err = ble_start_advertising();
        if err != 0 {
            zephyr::printkln!("BLE advertising failed: {}", err);
            return;
        }
    }
    
    zephyr::printkln!("BLE advertising started");
    
    loop {
        unsafe { 
            ble_sleep_ms(1000);
        }
    }
}

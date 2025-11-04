#![no_std]
#![allow(unexpected_cfgs)]

use log::warn;
use core::ffi::{c_int, c_long};

#[repr(C)]
pub struct Device {
	_private: [u8; 0],
}

extern "C" {
    fn ble_init() -> c_int;
    fn ble_start_advertising() -> c_int;
    fn sleep_ms(milliseconds: c_long);  
    fn get_light_value(dev: *const Device) -> c_int;
}

#[no_mangle]
extern "C" fn rust_ble_connected() {
    zephyr::printkln!("Rust: BLE connected!");
}

#[no_mangle]
extern "C" fn rust_ble_disconnected() {
    zephyr::printkln!("Rust: BLE disconnected!");
}

#[cfg(not(dt = "aliases::light_sensor"))]
fn do_sensor_init() -> Option<*const Device> {
	warn!("No light_sensor configured");
	return None;
}

#[cfg(dt = "aliases::light_sensor")]
fn do_sensor_init() -> Option<*const Device> {
	warn!("Going to initialize sensor");

	let mut tsl2591 = zephyr::devicetree::aliases::light_sensor::get_instance().ok()?;

	if !tsl2591.is_ready() {
		warn!("TSL2591 is not ready");
		return None;
	}

	Some(tsl2591.as_raw())
}

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }
    
    let tsl2591: *const Device;

    match do_sensor_init() {
	Some(s) => {
		zephyr::printkln!("TSL2591 is ready");
		tsl2591 = s;
	}
	None => {
		zephyr::printkln!("TSL2591 is not ready");
		return;
	}
    }

    zephyr::printkln!("Initializing BLE from Rust");
    
    unsafe {
        let err = ble_init();
        if err != 0 {
            zephyr::printkln!("BLE init failed: {}", err);
            return;
        }
        
        sleep_ms(100);
        
        let err = ble_start_advertising();
        if err != 0 {
            zephyr::printkln!("BLE advertising failed: {}", err);
            return;
        }
    }
    
    zephyr::printkln!("BLE advertising started");
    
    loop {
        unsafe { 
	    let light = get_light_value(tsl2591);
	    zephyr::printkln!("Light reading: {}", light);
            sleep_ms(100);
        }
    }
}

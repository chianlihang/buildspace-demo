#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(not(target_os = "macos"))]
use rppal::gpio::{Gpio, Level};

use std::{thread, time::Duration};
use tauri::{Emitter, Runtime};

const GPIO_SW: u8 = 10;
const GPIO_DT: u8 = 9;
const GPIO_CLK: u8 = 11;

#[cfg(not(target_os = "macos"))]
fn setup_rotary_encoder<F, R>(_app_handle: tauri::AppHandle<R>, callback: F)
where
    F: Fn(String) + Send + 'static,
    R: Runtime,
{
    let gpio = Gpio::new().unwrap();

    let sw = gpio.get(GPIO_SW).unwrap().into_input_pullup();
    let dt = gpio.get(GPIO_DT).unwrap().into_input_pullup();
    let clk = gpio.get(GPIO_CLK).unwrap().into_input_pullup();

    thread::spawn(move || {
        let mut last_clk = clk.read();
        let mut last_sw_state = sw.read();

        loop {
            let current_clk = clk.read();
            let current_dt = dt.read();
            let current_sw_state = sw.read();

            if current_clk != last_clk {
                if current_clk == Level::High {
                    if current_dt == last_clk {
                        callback("right".to_string());
                    } else {
                        callback("left".to_string());
                    }
                }
                last_clk = current_clk;
            }

            // Detect transition from High to Low (button press)
            if current_sw_state == Level::Low && last_sw_state == Level::High {
                callback("enter".to_string());
                // Wait for the button to be released
                while sw.read() == Level::Low {
                    thread::sleep(Duration::from_millis(1)); // debounce delay
                }
            }
            last_sw_state = current_sw_state;

            thread::sleep(Duration::from_millis(1)); // small delay to reduce CPU usage
        }
    });
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn start_encoder<R: Runtime>(app_handle: tauri::AppHandle<R>) {
    setup_rotary_encoder(app_handle.clone(), move |direction| {
        match direction.as_str() {
            "right" => app_handle.emit("rotate-right", ()).unwrap(),
            "left" => app_handle.emit("rotate-left", ()).unwrap(),
            "enter" => app_handle.emit("rotate-enter", ()).unwrap(),
            _ => {}
        }
    });
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn start_encoder<R: Runtime>(_app_handle: tauri::AppHandle<R>) {}

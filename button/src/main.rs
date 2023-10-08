use esp_idf_hal::gpio::PinDriver;
use std::{thread, time::Duration};

fn main() {
    let peripherals: esp_idf_hal::peripherals::Peripherals =
        esp_idf_hal::peripherals::Peripherals::take().unwrap();
    let gpios: esp_idf_hal::gpio::Pins = peripherals.pins;

    let pin_btn_a = PinDriver::input(gpios.gpio39).unwrap();
    let pin_btn_b = PinDriver::input(gpios.gpio38).unwrap();
    let pin_btn_c = PinDriver::input(gpios.gpio37).unwrap();

    let mut prev_btn_a_is_low = false;
    let mut prev_btn_b_is_low = false;
    let mut prev_btn_c_is_low = false;

    loop {
        // Print text if button A is low (= pressed) and was not low (= not pressed) in the previous iteration.
        if pin_btn_a.is_low() && !prev_btn_a_is_low {
            println!("Button A pressed");
            prev_btn_a_is_low = true;
        }

        // Set prev_btn_a_is_low to false if button A is high (= not pressed) and was low (= pressed) in the previous iteration.
        if pin_btn_a.is_high() && prev_btn_a_is_low {
            prev_btn_a_is_low = false;
        }

        // Implement same logics for button B and C.
        if pin_btn_b.is_low() && !prev_btn_b_is_low {
            println!("Button B pressed");
            prev_btn_b_is_low = true;
        }

        if pin_btn_b.is_high() && prev_btn_b_is_low {
            prev_btn_b_is_low = false;
        }

        if pin_btn_c.is_low() && !prev_btn_c_is_low {
            println!("Button C pressed");
            prev_btn_c_is_low = true;
        }

        if pin_btn_c.is_high() && prev_btn_c_is_low {
            prev_btn_c_is_low = false;
        }

        thread::sleep(Duration::from_millis(10));
    }
}

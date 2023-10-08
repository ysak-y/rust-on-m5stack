use esp_idf_hal::gpio::PinDriver;
use std::{thread, time::Duration};

macro_rules! process_button {
    ($pin:expr, $prev_state:expr, $button_name:expr) => {
        // Print text if button A is low (= pressed) and was not low (= not pressed) in the previous iteration.
        if $pin.is_low() && !$prev_state {
            println!("Button {} pressed", $button_name);
            $prev_state = true;
        // Set prev_btn_a_is_low to false if button A is high (= not pressed) and was low (= pressed) in the previous iteration.
        } else if $pin.is_high() && $prev_state {
            $prev_state = false;
        }
    };
}

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
        process_button!(pin_btn_a, prev_btn_a_is_low, "A");
        process_button!(pin_btn_b, prev_btn_b_is_low, "B");
        process_button!(pin_btn_c, prev_btn_c_is_low, "C");

        thread::sleep(Duration::from_millis(10));
    }
}

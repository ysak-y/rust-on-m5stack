use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Configuring output channel
    let peripherals = Peripherals::take().unwrap();
    let mut channel = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &config::TimerConfig::new().frequency(2.kHz().into()),
        )
        .unwrap(),
        peripherals.pins.gpio25,
    )
    .unwrap();

    FreeRtos::delay_ms(2000);

    let max_duty = channel.get_max_duty();
    println!("Max Duty: {}", max_duty);

    // Starting duty-cycle loop
    for numerator in [1, 2, 3].iter().cycle() {
        println!("Duty: {}", numerator);

        // Beep
        channel.set_duty(*numerator).unwrap();
        FreeRtos::delay_ms(500);

        // Stop beep
        channel.set_duty(0).unwrap();
        FreeRtos::delay_ms(2000);
    }
}

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::{config, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2};
use mipidsi::Builder;
use std::{thread, time::Duration};

fn main() {
    let peripherals = esp_idf_hal::peripherals::Peripherals::take().unwrap();
    let gpios = peripherals.pins;

    let pin_dc = PinDriver::output(gpios.gpio27).unwrap();
    let mut lcd_reset_pin = PinDriver::output(gpios.gpio33).unwrap();

    // Initialize back light of the display
    let mut pin_lcd_blk = PinDriver::output(gpios.gpio32).unwrap();
    pin_lcd_blk.set_high().unwrap();

    // Issue LCD Reset
    lcd_reset_pin.set_low().unwrap();
    thread::sleep(Duration::from_millis(100));
    lcd_reset_pin.set_high().unwrap();
    thread::sleep(Duration::from_millis(2000));

    // Initialize SPI interface
    let spi = peripherals.spi2;
    let driver = SpiDriver::new::<SPI2>(
        spi,
        gpios.gpio18,
        gpios.gpio23,
        Some(gpios.gpio19),
        &SpiDriverConfig::new(),
    )
    .unwrap();

    let spi_device_config = config::Config::new().baudrate(10.MHz().into());
    let spi_device = SpiDeviceDriver::new(driver, Some(gpios.gpio14), &spi_device_config).unwrap();

    // Initialize Display Interface
    let display_interface = SPIInterfaceNoCS::new(spi_device, pin_dc);

    // Initialize ILI9342c that is the M5Stack screen.
    let mut display = Builder::ili9342c_rgb565(display_interface)
        .with_display_size(320, 240)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut esp_idf_hal::delay::FreeRtos, Some(lcd_reset_pin))
        .unwrap();

    // Draw 5x5 rectangle at position (0, 0).
    display
        .set_pixels(
            0,
            0,
            5,
            5,
            core::iter::repeat(Rgb565::RED).take(25).into_iter(),
        )
        .unwrap();

    // Draw text at position (50, 50)
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);
    Text::with_alignment(
        "Hello World!!",
        Point::new(50, 50),
        style,
        Alignment::Center,
    )
    .draw(&mut display)
    .unwrap();
}

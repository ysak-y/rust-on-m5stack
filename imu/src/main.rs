use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use mpu6886::Mpu6886;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();
    let mut mpu = Mpu6886::new(i2c);
    let _ = mpu.init(&mut FreeRtos);

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles().unwrap();
        println!("r/p: {:?}", acc);

        // get temp
        let temp = mpu.get_temp().unwrap();
        println!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().unwrap();
        println!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().unwrap();
        println!("acc: {:?}", acc);

        FreeRtos::delay_ms(1000);
    }
}

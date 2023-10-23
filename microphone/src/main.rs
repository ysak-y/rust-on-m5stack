use esp_idf_hal::{
    delay::FreeRtos,
    i2s::{
        config::{
            Config, DataBitWidth, PdmRxClkConfig, PdmRxConfig, PdmRxGpioConfig, PdmRxSlotConfig,
            SlotMode,
        },
        I2sDriver, I2sRx,
    },
    prelude::Peripherals,
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let pdm_config = PdmRxConfig::new(
        Config::default(),
        PdmRxClkConfig::from_sample_rate_hz(44100),
        PdmRxSlotConfig::from_bits_per_sample_and_slot_mode(DataBitWidth::Bits16, SlotMode::Mono),
        PdmRxGpioConfig::new(false),
    );
    let peripherals = Peripherals::take().unwrap();
    let din = peripherals.pins.gpio21;
    let clk = peripherals.pins.gpio22;

    // An I2S bus that communicates in standard or TDM mode consists of the following lines:
    //
    // MCLK: Master clock line. It is an optional signal depending on the slave side, mainly used for offering a reference clock to the I2S slave device.
    // BCLK: Bit clock line. The bit clock for data line.
    // WS: Word (Slot) select line. It is usually used to identify the vocal tract except PDM mode.
    // DIN/DOUT: Serial data input/output line. Data will loopback internally if DIN and DOUT are set to a same GPIO.
    let mut i2s = I2sDriver::<I2sRx>::new_pdm_rx(peripherals.i2s0, &pdm_config, clk, din).unwrap();
    i2s.rx_enable().unwrap();

    loop {
        let mut buf: [u8; 4096] = [0; 4096];
        let num_bytes_read = i2s.read(&mut buf, 2000).unwrap();
        println!("{} bytes read", num_bytes_read);
        println!("buf is {:?}", buf);
        FreeRtos::delay_ms(1000);
    }
}

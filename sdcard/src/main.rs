use embedded_sdmmc::{TimeSource, Timestamp};
use esp_idf_hal::{
    gpio::PinDriver,
    peripherals,
    prelude::*,
    spi::{config, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

struct Clock;
impl TimeSource for Clock {
    fn get_timestamp(&self) -> Timestamp {}
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = peripherals::Peripherals::take().unwrap();
    let gpios = peripherals.pins;
    let pin_cs = PinDriver::output(gpios.gpio4).unwrap();

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

    // Build an SD Card interface out of an SPI device, a chip-select pin and a delay object
    let sdcard = embedded_sdmmc::SdCard::new(spi_device, pin_cs, esp_idf_hal::delay::FreeRtos);
    // Get the card size (this also triggers card initialisation because it's not been done yet)
    println!("Card size is {} bytes", sdcard.num_bytes().unwrap());
    // Now let's look for volumes (also known as partitions) on our block device.
    // To do this we need a Volume Manager. It will take ownership of the block device.
    let mut volume_mgr = embedded_sdmmc::VolumeManager::new(sdcard, time_source);
    // Try and access Volume 0 (i.e. the first partition).
    // The volume object holds information about the filesystem on that volume.
    // It doesn't hold a reference to the Volume Manager and so must be passed back
    // to every Volume Manager API call. This makes it easier to handle multiple
    // volumes in parallel.
    let volume0 = volume_mgr.get_volume(embedded_sdmmc::VolumeIdx(0))?;
    println!("Volume 0: {:?}", volume0);
    // Open the root directory (passing in the volume we're using).
    let root_dir = volume_mgr.open_root_dir(&volume0)?;
    // Open a file called "MY_FILE.TXT" in the root directory
    let my_file = volume_mgr
        .open_file_in_dir(root_dir, "MY_FILE.TXT", embedded_sdmmc::Mode::ReadOnly)
        .unwrap();
    // Print the contents of the file
    while !volume_mgr.file_eof(my_file).unwrap() {
        let mut buffer = [0u8; 32];
        let num_read = volume_mgr.read(&volume0, &mut my_file, &mut buffer)?;
        for b in &buffer[0..num_read] {
            print!("{}", *b as char);
        }
    }
    volume_mgr.close_file(&volume0, my_file);
    volume_mgr.close_dir(&volume0, root_dir);
}

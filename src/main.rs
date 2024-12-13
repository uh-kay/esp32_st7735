use embedded_graphics::{image::{Image, ImageRaw, ImageRawLE}, mono_font::{iso_8859_1::FONT_6X10, MonoTextStyle}, pixelcolor::Rgb565, prelude::*, text::Text};
use esp_idf_hal::{delay::Delay, gpio::{self, PinDriver}, peripherals::Peripherals, spi::{config::Config, SpiDeviceDriver, SpiDriver, SpiDriverConfig}};
use esp_idf_svc::{log::EspLogger, sys};
use st7735_lcd::Orientation;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let dc = PinDriver::output(peripherals.pins.gpio27).unwrap();

    let sclk = peripherals.pins.gpio23;
    let mosi = peripherals.pins.gpio5;
    let cs = peripherals.pins.gpio26;
    let rst = PinDriver::output(peripherals.pins.gpio21).unwrap();

    let spi_driver = SpiDriver::new(peripherals.spi2, sclk, mosi, None::<gpio::AnyIOPin>, &SpiDriverConfig::new()).unwrap();

    let config = Config::new();

    let spi = SpiDeviceDriver::new(spi_driver, Some(cs), &config).unwrap();

    let mut delay = Delay::default();

    let mut display = st7735_lcd::ST7735::new(spi, dc, rst, false, false, 128, 128);

    display.init(&mut delay).unwrap();
    
    display.set_orientation(&Orientation::PortraitSwapped).unwrap();
    display.set_offset(0, 0);

    println!("lcd test have done.");
    loop {
        display.clear(Rgb565::BLACK).unwrap();
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

        Text::new("Hello Rust!", Point::new(20, 30), style).draw(&mut display).unwrap();
        

        let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);
        let image = Image::new(&image_raw, Point::new(26, 50));
        image.draw(&mut display).unwrap();
        delay.delay_ms(5000);
    }
}

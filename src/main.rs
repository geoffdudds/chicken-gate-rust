#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*, system::SystemControl,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0, &clocks);
    let _init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, system::SystemControl};
// use esp_hal_embassy;
// use esp_println::print;

// #[entry]
// fn main() -> ! {
//     let peripherals = Peripherals::take();
//     let system = SystemControl::new(peripherals.SYSTEM);

//     let clocks = ClockControl::max(system.clock_control).freeze();
//     let delay = Delay::new(&clocks);

//     esp_println::logger::init_logger_from_env();

//     let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0, &clocks);
//     let _init = esp_wifi::initialize(
//         esp_wifi::EspWifiInitFor::Wifi,
//         timg0.timer0,
//         esp_hal::rng::Rng::new(peripherals.RNG),
//         peripherals.RADIO_CLK,
//         &clocks,
//     )
//     .unwrap();

//     loop {
//         log::info!("Hello world!");
//         delay.delay(500.millis());
//     }
// }

#[embassy_executor::task]
async fn one_second_task() {
    let mut count = 0;

    loop {
        esp_println::println!("Spawn Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    esp_hal_embassy::init(
        &clocks,
        esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    spawner.spawn(one_second_task()).unwrap();

    let mut count = 0;
    loop {
        esp_println::println!("Main Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

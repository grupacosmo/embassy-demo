#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::result;
use defmt_rtt as _;
use embassy_executor::{SpawnError, Spawner};
use embassy_stm32::{
    gpio::{AnyPin, Level, Output, Pin, Speed},
    Peripherals,
};
use embassy_time::{Timer, Duration};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let device = embassy_stm32::init(Default::default());

    if let Err(e) = init(&spawner, device) {
        defmt::error!("{}", e)
    }
}

fn init(spawner: &Spawner, device: Peripherals) -> Result<()> {
    let led_pin = device.PB14.degrade();
    spawner.spawn(blink(led_pin))?;
    Ok(())
}

#[embassy_executor::task]
async fn blink(led_pin: AnyPin) {
    let mut led = Output::new(led_pin, Level::Low, Speed::VeryHigh);

    loop {
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[derive(Debug, derive_more::From)]
enum Error {
    #[from]
    Spawn(SpawnError),
}

impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::Spawn(e) => defmt::write!(fmt, "Failed to spawn a task {}", e),
        }
    }
}

type Result<T> = result::Result<T, Error>;

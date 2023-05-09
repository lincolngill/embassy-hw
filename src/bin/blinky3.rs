/*
Blink 3 LEDs
*/
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Pin, Level, Output};
use embassy_rp::peripherals::{PIN_25};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Main");
    let p = embassy_rp::init(Default::default());
    let p25: Output<'static, PIN_25> = Output::new(p.PIN_25, Level::Low);
    unwrap!(spawner.spawn(led_blinker(p25,Duration::from_millis(500))));
    // Cannot spawn the same async function/task multiple times.
    unwrap!(spawner.spawn(anypin_blinker1(p.PIN_14.degrade())));
    unwrap!(spawner.spawn(anypin_blinker2(p.PIN_15.degrade(),Duration::from_secs(1),Duration::from_millis(300))));
    info!("Main Done");
}

#[embassy_executor::task]
async fn led_blinker(mut led: Output<'static, PIN_25>, interval:Duration) {
    info!("Pin 25 LED toggle {} ms", interval.as_millis());
    loop {
        led.toggle();
        Timer::after(interval).await;
    }
}

#[embassy_executor::task]
async fn anypin_blinker1(pin: AnyPin) {
    // Blink any pin at fixed rate (300ms on, 500ms off).
    info!("Pin {} on/off 300/500 ms", pin.pin());
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;

        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn anypin_blinker2(pin: AnyPin, on: Duration, off: Duration) {
    //Blink any pin at specific on and off duration.
    info!("Pin {} on/off {}/{} ms", pin.pin(), on.as_millis(), off.as_millis());
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.set_high();
        Timer::after(on).await;

        led.set_low();
        Timer::after(off).await;
    }
}
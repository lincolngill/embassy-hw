#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Pin};
use {defmt_rtt as _, panic_probe as _};

use embassy_hw::button::GndButton;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    debug!("Main");
    let p = embassy_rp::init(Default::default());
    unwrap!(spawner.spawn(button_task(p.PIN_15.degrade())));
    debug!("Main task done.");
}

#[embassy_executor::task]
async fn button_task(button_pin: AnyPin) {
    let mut button = GndButton::new(button_pin);
    debug!("Push the button...");
    loop {
        button.wait_for_push_release().await;
        debug!("Ping");
    }
}

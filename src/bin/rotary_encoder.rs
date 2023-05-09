#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use {defmt_rtt as _, panic_probe as _};

use embassy_hw::button::GndButton;
use embassy_hw::rotary_encoder::{ Encoder, Direction::* };

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    debug!("Main");
    let p = embassy_rp::init(Default::default());
    let encoder: Encoder<'static> = Encoder::new(
        p.PIN_17.degrade(),
        p.PIN_16.degrade(),
    );
    let encoder_button: GndButton<'static> = GndButton::new(p.PIN_15.degrade());
    unwrap!(spawner.spawn(encoder_button_task(encoder_button)));
    unwrap!(spawner.spawn(encoder_task(encoder)));
    debug!("Main task done.");
}

#[embassy_executor::task]
async fn encoder_button_task(mut button: GndButton<'static>) {
    debug!("Push the encoder button...");
    loop {
        button.wait_for_push_release().await;
        debug!("Encoder button Ping");
    }
}

#[embassy_executor::task]
async fn encoder_task(mut encoder: Encoder<'static>) {
    debug!("Turn the encoder...");
    loop {
        let direction = encoder.rotation().await;
        match direction {
            Clockwise => debug!("Encoding clockwise"),
            AntiClockwise => debug!("Encoding anti-clockwise"),
        };
    }
}

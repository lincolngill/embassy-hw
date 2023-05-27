// pio ws2812 led strip.
// Press the encoder button to toggle LED blinking on or off.
// Turn the encoder to increase or decrease the LED blinking frequency.
//
// Pin 25 = LED
// Rotary Encoder:
//   https://www.jaycar.co.nz/rotary-encoder-switch-with-pushbutton/p/SR1230
//   Pin 15 = Button
//   Pin 17 = A
//   Pin 16 = B
//
// A multi tasking example with shared state.
// Refer: https://apollolabsblog.hashnode.dev/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives
//
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Level, Output, Pin};
use embassy_rp::pio::Pio;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use embassy_hw::button::GndButton;
use embassy_hw::rotary_encoder::{Direction::*, Encoder};
use embassy_hw::ws2812::Ws2812;
use smart_leds::RGB8;

/// LED shared state
// Derive Copy and Clone, so tasks can copy the state and release the Mutex lock
#[derive(Copy, Clone)]
struct LedState {
    ms: u64,
    toggle: bool,
}

// Initial LED_STATE is 512ms blink toggle time and LED toggling is off.
static LED_STATE: Mutex<ThreadModeRawMutex, LedState> = Mutex::new(LedState {
    ms: 512,
    toggle: false,
});
//const LED_MS_ADJUST: u64 = 50;

fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}

/// Main task initialises the Rotary encoder and LED. Then task completes.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    debug!("Main");
    let p = embassy_rp::init(Default::default());

    let Pio { common, sm0, .. } = Pio::new(p.PIO0);
    const NUM_LEDS: usize = 120;
    let mut data = [RGB8::default(); NUM_LEDS];
    let mut ws2812 = Ws2812::new(common, sm0, p.PIN_8);

    let encoder: Encoder<'static> = Encoder::new(p.PIN_17.degrade(), p.PIN_16.degrade());
    let encoder_button: GndButton<'static> = GndButton::new(p.PIN_15.degrade());
    unwrap!(spawner.spawn(encoder_button_task(encoder_button)));
    unwrap!(spawner.spawn(encoder_task(encoder)));
    unwrap!(spawner.spawn(led_blinker(p.PIN_25.degrade())));
    let mut stripon = false;
    loop {
        // This block will lock LED_STATE until it's copied to led_state.
        let led_state = {
            let led_state = LED_STATE.lock().await;
            *led_state
        }; // Mutex lock dropped here.
        if led_state.toggle {
            stripon = true;
            for j in 0..(256 * 5) {
                //            debug!("New Colors:");
                for i in 0..NUM_LEDS {
                    data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                    //               debug!("R: {} G: {} B: {}", data[i].r, data[i].g, data[i].b);
                }
                ws2812.write(&data).await;
            }
            Timer::after(Duration::from_secs(3)).await;
        } else {
            if stripon {
                for i in 0..NUM_LEDS {
                    data[i] = RGB8 { r: 0, g: 0, b: 0 };
                }
                ws2812.write(&data).await;
                stripon = false;
            }
            Timer::after(Duration::from_millis(500)).await;
        }
    }
    //debug!("Main task done.");
}

/// When the rotary encoder button is pressed, toggle the LED blinking on or off.
#[embassy_executor::task]
async fn encoder_button_task(mut button: GndButton<'static>) -> ! {
    debug!("Push the encoder button...");
    loop {
        button.wait_for_push_release().await;
        let mut led_state = LED_STATE.lock().await;
        // Toggle the toggle state.
        led_state.toggle = !led_state.toggle;
        debug!("LED toggle={}", led_state.toggle);
    }
}

/// When the rotary encoder is turned, increase or decrease the LED blinking frequency.
/// LED_STATE.ms will be updated, even if LED_STATE.toggle = false.
#[embassy_executor::task]
async fn encoder_task(mut encoder: Encoder<'static>) -> ! {
    debug!("Turn the encoder...");
    loop {
        let direction = encoder.rotation().await;
        let mut led_state = LED_STATE.lock().await;
        match direction {
            Clockwise => {
                debug!("Encoding clockwise");
                //match led_state.ms.checked_add(LED_MS_ADJUST) {
                led_state.ms = match led_state.ms.checked_mul(2) {
                    Some(v) => {
                        if v == 0 {
                            1
                        } else {
                            v
                        }
                    }
                    None => u64::MAX,
                };
            }
            AntiClockwise => {
                debug!("Encoding anti-clockwise");
                //match led_state.ms.checked_sub(LED_MS_ADJUST) {
                led_state.ms = match led_state.ms.checked_div(2) {
                    Some(v) => v,
                    None => 0,
                };
            }
        };
        debug!("LED ms = {}", led_state.ms);
    }
}

/// If LED_STATE.toggle is true, toggle the LED on and off for LED_STATE.ms.
#[embassy_executor::task]
async fn led_blinker(led_pin: AnyPin) -> ! {
    debug!("LED task started");
    let mut pin_output = Output::new(led_pin, Level::Low);
    loop {
        // This block will lock LED_STATE until it's copied to led_state.
        let led_state = {
            let led_state = LED_STATE.lock().await;
            *led_state
        }; // Mutex lock dropped here.
           // Wait 500ms if LED_STATE.on = false.
        let mut ms: u64 = 500;
        if led_state.toggle {
            pin_output.toggle();
            ms = led_state.ms;
        }
        Timer::after(Duration::from_millis(ms)).await;
    }
}
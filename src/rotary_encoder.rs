use embassy_rp::gpio::{AnyPin, Input, Pull};
use embassy_time::{Duration, Timer};
//use defmt::*;

pub struct Encoder<'a> {
    a_input: Input<'a, AnyPin>,
    b_input: Input<'a, AnyPin>,
}

pub enum Direction {
    Clockwise,
    AntiClockwise,
}

impl<'a> Encoder<'a> {
    pub fn new(a_pin: AnyPin, b_pin: AnyPin) -> Encoder<'a> {
        let a_input = Input::new(a_pin, Pull::Up);
        let b_input = Input::new(b_pin, Pull::Up);
        Encoder { a_input, b_input }
    }

    pub async fn rotation(&mut self) -> Direction {
        self.a_input.wait_for_any_edge().await;
        Timer::after(Duration::from_millis(10)).await;
        let a: bool = self.a_input.get_level().into();
        let b: bool = self.b_input.get_level().into();
        //debug!("a={} b={}", a, b);
        if a == b {
            return Direction::Clockwise;
        }
        Direction::AntiClockwise
    }
}

use embassy_rp::gpio::{AnyPin, Input, Pull};
pub struct GndButton<'a> {
    async_input: Input<'a, AnyPin>,
}

impl<'a> GndButton<'a> {
    pub fn new(pin: AnyPin) -> GndButton<'a> {
        let async_input =  Input::new(pin, Pull::Up);
        GndButton { async_input }
    }

    pub async fn wait_for_push(&mut self) {
        self.async_input.wait_for_low().await;
    }

    pub async fn wait_for_push_release(&mut self) {
        // Probably only need to wait for high!
        self.async_input.wait_for_low().await;
        self.async_input.wait_for_high().await;
    }
}
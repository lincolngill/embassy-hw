# Embedded rust Basic Hardware Drivers & Examples

Embedded rust embassy hardware library, and examples, for Raspberry Pico RP2040

## Driver modules:
* Button - GndButton struct for a pull-up input button. Press to ground.
* RotaryEncoder - 3 pin rotary encoder driver.

## Example binary crates
* button.rs - Press a button.
* rotary_encoder.rs - Turn a knob.
* rotary_led.rs - Control an LED blinking frequency with a buttoned rotary encoder. E.g. multiple tasks with shared state. 


This is an out-of-tree crate project, using https://github.com/embassy-rs 20-Apr-2023 [patch.crate-io] commit.

## Setup & Run

```
rustup default nightly
rustup update
rustup target add thumbv6m-none-eabi

cargo install probe-rs-cli
```

### Run

```
cargo run 
or
cargo run --release
```
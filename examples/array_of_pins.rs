#![no_std]
#![no_main]

use bl602_hal as hal;
use core::convert::Infallible;
use embedded_hal::digital::blocking::{InputPin, OutputPin};
use hal::{
    prelude::*,
    pac,
};
use panic_halt as _;

/// Example of ways to use pins with generification, indexing, iteration
#[riscv_rt::entry]
fn main() -> ! {
    //take control of the device peripherals:
    let dp = pac::Peripherals::take().unwrap();
    let gpio_pins = dp.GLB.split();

    /*
        Generics
    */

    // Initialize the pins to a default state
    let mut p0 = gpio_pins.pin0.into_pull_down_output();
    let mut p1 = gpio_pins.pin1.into_pull_down_output();

    // generic function with trait constraint
    do_a_thing_with_a_pin(&mut p0);
    do_a_thing_with_a_pin(&mut p1);

    /*
        Dynamic trait objects
    */

    // Initialize pins to a default state
    let mut p2 = gpio_pins.pin2.into_pull_down_output();
    let mut p3 = gpio_pins.pin3.into_pull_down_output();
    let mut p4 = gpio_pins.pin4.into_pull_down_output();

    // store as reference to dyn trait object
    let mut pins: [&mut dyn OutputPin<Error = Infallible>; 3] = [&mut p2, &mut p3, &mut p4];

    pins[0].set_high().ok();

    for pin in pins.iter_mut() {
        pin.set_low().ok();
    }

    /*
        Type Erasure:
    */

    // Initialize the pins to a default state, then erase the type:
    let p5 = gpio_pins.pin5.into_pull_down_output().erase();
    let p6 = gpio_pins.pin6.into_pull_down_output().erase();
    let p7 = gpio_pins.pin7.into_pull_down_output().erase();

    let p8 = gpio_pins.pin8.into_pull_down_input().erase();
    let p9 = gpio_pins.pin9.into_pull_down_input().erase();
    let p10 = gpio_pins.pin10.into_pull_down_input().erase();

    // can be stored as owned or reference
    let mut output_pins = [p5, p6, p7];

    // inputs and outputs both supported
    let input_pins = [p8, p9, p10];

    if let Ok(_) = input_pins[1].is_high() {
        pins[0].set_high().ok();
    }

    for pin in output_pins.iter_mut() {
        pin.set_low().ok();
    }

    loop {

    }
}

fn do_a_thing_with_a_pin(pin: &mut impl OutputPin) {
    pin.set_high().ok();
    pin.set_low().ok();
}
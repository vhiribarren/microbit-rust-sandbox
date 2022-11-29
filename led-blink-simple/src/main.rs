#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal;
use microbit::hal::prelude::*;
use panic_halt as _;

const DELAY: u32 = 1000;

#[entry]
fn main() -> ! {

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut gpio = board.pins.p0_02.into_push_pull_output(hal::gpio::Level::Low);
    // pO_O2 is P0 on Edge connector

    loop {
        gpio.set_high().unwrap();
        timer.delay_ms(DELAY);
        gpio.set_low().unwrap();
        timer.delay_ms(DELAY);
    }
    
}

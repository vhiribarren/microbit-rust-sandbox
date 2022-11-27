#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal;
use microbit::hal::prelude::*;


#[entry]
fn main() -> ! {

    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut temp = hal::temp::Temp::new(board.TEMP);
    let mut timer = Timer::new(board.TIMER0);

    loop {
        let temp_value= temp.measure();
        rprintln!("Temperature: {}", temp_value);
        timer.delay_ms(30000u32);
    }
    
}

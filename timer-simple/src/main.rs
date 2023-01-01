#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_timer_CountDown;
use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    timer.start(core::u32::MAX);
    
    loop {
        if timer.read() > 1_000_000 {
            rprintln!("1 second");
            timer.start(core::u32::MAX);
        }
    }
}

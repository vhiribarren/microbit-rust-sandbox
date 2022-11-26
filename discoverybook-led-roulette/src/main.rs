#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;


const STEP_DURATION_MS: u32 = 100;
const EMPTY_ARRAY: [[u8; 5]; 5] =  [[0; 5]; 5];


#[entry]
fn main() -> ! {

    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut position = (0_usize, 0_usize);

    loop {
        let mut lights = EMPTY_ARRAY;
        lights[position.1][position.0] = 1;
        display.show(&mut timer, lights, STEP_DURATION_MS);
        match position {
            (x, 0) if x < 4 => position.0 += 1,
            (4, y) if y < 4 => position.1 += 1,
            (x, 4) if x > 0 => position.0 -= 1,            
            (0, y) if y > 0 => position.1 -= 1,
            other => rprintln!("This position should not happen: {:?}", other),
        }
    }
    
}

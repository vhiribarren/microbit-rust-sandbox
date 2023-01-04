#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use microbit::hal::pac::interrupt;
use microbit::hal::prelude::*;
use microbit::hal::timer::Timer;
use microbit::hal::Delay;
use microbit::{board::Board, pac::TIMER0};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

static STATIC_TIMER: Mutex<RefCell<Option<Timer<TIMER0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();

    unsafe {
        microbit::pac::NVIC::unmask(interrupt::TIMER0);
    }

    let board = Board::take().unwrap();
    let mut delay = Delay::new(board.SYST);

    let mut timer = Timer::new(board.TIMER0);
    timer.enable_interrupt();
    timer.start(2_000_000_u32);

    cortex_m::interrupt::free(|cs| {
        STATIC_TIMER.borrow(cs).replace(Some(timer));
    });

    rprintln!("Start delay...");
    delay.delay_ms(5_000_u32);
    rprintln!("End of delay.");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[interrupt]
fn TIMER0() {
    rprintln!("TIMER0 interruption!");
    cortex_m::interrupt::free(|cs| {
        STATIC_TIMER
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .disable_interrupt();
    });
}

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal;
use microbit::hal::prelude::*;
use microbit::hal::pwm::Pwm;
use panic_halt as _;

const SOUND_FREQUENCY: u32 = 1000;
const SOUND_VOLUME: f32 = 0.7;

#[entry]
fn main() -> ! {

    let board = Board::take().unwrap();

    let speaker_pin = board
        .speaker_pin
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();

    let mut pwm = Pwm::new(board.PWM0);
    pwm.set_output_pin(hal::pwm::Channel::C0, speaker_pin);
    pwm.set_period(SOUND_FREQUENCY.hz());

    let max_volume = pwm.get_max_duty()/2;
    let volume = (SOUND_VOLUME * (max_volume as f32)) as u16;

    pwm.set_duty(hal::pwm::Channel::C0, volume);

    loop {}
}

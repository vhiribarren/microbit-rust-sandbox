#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal;
use microbit::hal::prelude::*;
use microbit::hal::pwm::{Instance, Pwm};
use microbit::hal::time::Hertz;
use panic_halt as _;


const SOUND_VOLUME: f32 = 0.7;

struct Note;

impl Note {
    const DO_1: Hertz = Hertz(523);
    const RE_1: Hertz = Hertz(587);
    const MI_1: Hertz = Hertz(659);
    const FA_1: Hertz = Hertz(698);
    const SOL_1: Hertz = Hertz(784);
    const LA_1: Hertz = Hertz(880);
    const SI_1: Hertz = Hertz(988);
    const DO_2: Hertz = Hertz(1046);
}

const NOTES: [Hertz; 8] = [
    Note::DO_1,
    Note::RE_1,
    Note::MI_1,
    Note::FA_1,
    Note::SOL_1,
    Note::LA_1,
    Note::SI_1,
    Note::DO_2,
];

fn play_note<T: Instance>(pwm: &mut Pwm<T>, freq: Hertz, volume: f32) {
    pwm.set_period(freq);
    let max_volume = pwm.get_max_duty() / 2;
    let duty = (volume * (max_volume as f32)) as u16;
    pwm.set_duty(hal::pwm::Channel::C0, duty);
}

#[entry]
fn main() -> ! {

    let board = Board::take().unwrap();
    let mut timer = hal::timer::Timer::new(board.TIMER0);

    let speaker_pin = board
        .speaker_pin
        .into_push_pull_output(hal::gpio::Level::High)
        .degrade();

    let mut pwm = Pwm::new(board.PWM0);
    pwm.set_output_pin(hal::pwm::Channel::C0, speaker_pin);

    loop {
        for note in NOTES {
            play_note(&mut pwm, note, SOUND_VOLUME);
            timer.delay_ms(1000u32);
        }
    }
}

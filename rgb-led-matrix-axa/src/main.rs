/*
MIT License

Copyright (c) 2022, 2023 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#![no_main]
#![no_std]

use core::mem;

use cortex_m_rt::entry;
use nrf52833_hal::pac::Peripherals;
use nrf52833_rgb_led_matrix::{
    canvas::{Canvas, Color},
    fonts::Font5x7,
    init_scheduled_led_matrix_system, register_panic_handler_with_logging,
};

const LOGO_DATA: &[u8; 3072] = include_bytes!("logo.in");

#[entry]
fn main() -> ! {
    register_panic_handler_with_logging!();
    let peripherals = Peripherals::take().unwrap();
    let scheduled_led_matrix = init_scheduled_led_matrix_system!(peripherals);

    let logo_array = unsafe { mem::transmute::<&[u8; 3072], &[[Color; 32]; 32]>(LOGO_DATA) };
    let logo_canvas = Canvas::<32, 32>(*logo_array);

    cortex_m::interrupt::free(|cs| {
        let mut borrowed_scheduled_led_matrix = scheduled_led_matrix.borrow(cs).borrow_mut();
        let led_matrix = borrowed_scheduled_led_matrix.as_mut().unwrap();
        let canvas = led_matrix.borrow_mut_canvas();
        canvas.draw_canvas(0, 0, &logo_canvas, Default::default());
        canvas.draw_text(36, 0, "GETD", Font5x7, Default::default());
        canvas.draw_text(39, 8, "TEX", Font5x7, Default::default());
        canvas.draw_text(33, 17, "LILLE", Font5x7, Default::default());
        canvas.draw_text(36, 25, "DEPT", Font5x7, Default::default());
    });

    loop {
        cortex_m::asm::wfi();
    }
}

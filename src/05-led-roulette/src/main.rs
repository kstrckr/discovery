#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

fn clean_tics(tic_ms: u16, leds: &mut LedArray, delay: &mut Delay) {
    for i in 0..8 {
        // let body = i;
        let head = if i < 7 { i + 1 } else { 0 };
        let tail = if i > 0 { i - 1 } else { 7 };
        // leds[body].on().ok();
        leds[tail].off().ok();
        delay.delay_ms(tic_ms);
        leds[head].on().ok();
        delay.delay_ms(tic_ms);
    }
}

fn pub_solution(ms: u16, leds: &mut LedArray, delay: &mut Delay) {
    for curr in 0..8 {
        let next = (curr + 1) % 8;

        leds[next].on().ok();
        delay.delay_ms(ms);
        leds[curr].off().ok();
        delay.delay_ms(ms);
    }
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let tic = 50_u16;
    loop {
        clean_tics(tic, &mut leds, &mut delay);
        // pub_solution(tic, &mut leds, &mut delay);
    }
}


#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

fn clean_tics(tic_ms: u16, leds: &mut LedArray, delay: &mut Delay) {
    for i in 0..8 {
        let body = i;
        let head = if i < 7 { i + 1 } else { 0 };
        let tail = if i > 0 { i - 1 } else { 7 };
        // leds[body].on().ok();
        delay.delay_ms(tic_ms);
        leds[head].on().ok();
        leds[tail].off().ok();
    }
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let tic = 50_u16;
    loop {
        clean_tics(tic, &mut leds, &mut delay);
    }
}


#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{Delay, DelayMs, LedArray, OutputSwitch, entry};

fn tic(leds: &mut LedArray, delay: &mut Delay) {

    let interval: u16 = 50;
    let overlap: u16 = 25;

    for i in 0..8 {
        leds[i].on().ok();
        delay.delay_ms(interval);
        if i == 7 {
            leds[0].on().ok();
        } else {
            leds[i+1].on().ok();
        }
        delay.delay_ms(overlap);
        leds[i].off().ok();
    }
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut half_period = 500_u16;
    let v_half_period = Volatile::new(&mut half_period);

    loop {
        tic(&mut leds, &mut delay);
    }
}


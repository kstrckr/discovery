#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux999::{entry, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));

    // CEN: Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // Wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}

    // Clear the update event flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (gpioa, rcc, tim6) = aux999::init();

    // Power on the TIM6 timer
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    // OPM Select one pulse mode
    // CEN Keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // Configure the prescaler to have the counter operate at 1 KHz
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 MHz / (7999 + 1) = 1 KHz
    // The counter (CNT) will increase on every millisecond
    tim6.psc.write(|w| w.psc().bits(7_999));
    let ms = 25;

    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());

    gpioa.moder.modify(|_, w| {
        w.moder1().output()
    });


    // aux999::bkpt();

    loop {
        // Turn on all the GPIO pins you need
        gpioa.odr.write(|w| {
            w.odr1().bit(true)
        });
        delay(tim6, ms);

        gpioa.odr.write(|w| {
            w.odr1().bit(false)
        });
        delay(tim6, ms);
    }
}

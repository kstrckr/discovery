#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*};
pub use lsm303agr::{AccelOutputDataRate, Lsm303agr};

#[entry]
fn main() -> ! {
    let (_leds, mut delay, mut sensor, mut itm) = aux15::init();


    loop {
        // iprintln!(&mut itm.stim[0], "{:?}", lsm303dlhc.mag().unwrap());
        // delay.delay_ms(1_000_u16);
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();

            // format!("Acceleration: x {:?} y {:?} z {:?}", data.x, data.y, data.z);
            // iprintln!("Acceleration: x {:?} y {:?} z {:?}", data.x, data.y, data.z);
            iprintln!(&mut itm.stim[0], "{} - {} - {}", data.x, data.y, data.z);
        }
    }
}

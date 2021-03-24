mod sensor;

use crate::sensor::sensor::Ultrasonic;
use std::time::Duration;
use std::thread::sleep;

use rust_gpiozero::*;

fn main() {
    let mag         = InputDevice::new(23);
	let mag_led 	    = LED::new(25);
	let ppl_led 	    = LED::new(18);

    mag_led.on();
    ppl_led.on();

    let mut ultrasonic = Ultrasonic::new(5, 6);

    loop {    
        println!("reading: {}", ultrasonic.get_median_reading());
    }
}

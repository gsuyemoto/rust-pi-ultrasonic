mod sensor;

use crate::sensor::sensor::Ultrasonic;
use std::time::Duration;
use std::thread::sleep;

use rust_gpiozero::*;

fn main() {
    let mag         = InputDevice::new(23);
    let mut trig    = OutputDevice::new(5);
    let echo        = InputDevice::new(6);
	let mag_led 	    = LED::new(25);
	let ppl_led 	    = LED::new(18);

    mag_led.on();
    ppl_led.on();

    let mut ultrasonic = Ultrasonic::new(trig, echo);
    ultrasonic.start();

    sleep(Duration::from_secs(1));
    println!("reading: {}", ultrasonic.get_reading());

    ultrasonic.stop();
}

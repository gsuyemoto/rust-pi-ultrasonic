extern crate sensors;

use sensors::ultrasonic::Ultrasonic;

pub trait DetectPerson {
    fn person_detected(&mut self, min_distance: u128) -> bool;
}

impl DetectPerson for Ultrasonic {
    fn person_detected(&mut self, min_distance: u128) -> bool {
        self.get_median_reading() < min_distance
    }
}

fn main() {
    let mut ultrasonic = Ultrasonic::new(5, 6);

    loop {    
        println!("reading: {}", ultrasonic.person_detected(30));
    }
}

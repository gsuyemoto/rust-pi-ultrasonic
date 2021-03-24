mod sensor;
use crate::sensor::sensor::Ultrasonic;

trait DetectPerson {
    fn person_detected(&mut self) -> bool;
}

impl DetectPerson for Ultrasonic {
    fn person_detected(&mut self) -> bool {
        self.get_median_reading() < 40
    }
}

fn main() {
    let mut ultrasonic = Ultrasonic::new(5, 6);

    loop {    
        println!("reading: {}", ultrasonic.person_detected());
    }
}

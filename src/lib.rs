pub mod ultrasonic {
    use std::time::{Duration, Instant};
    use rust_gpiozero::*;

    const MEDIAN_READINGS: usize    = 11;
    const MEDIAN_INDEX: usize       = 6;
    const DIVISOR_INCHES: u128      = 148;
    // const DIVISOR_CM: u128          = 148; // in centimeters 
    const MAX_IN_INCHES: u128       = 196; // max distance of sonar
    // const MAX_IN_CM: u128           = 500; // max distance in centimeters
    const TIME_TO_FAIL: u128        = 1700;
    const TIME_NEXT_READING: u64    = 5;
    
    pub struct Ultrasonic {
        pin_out:    u8,
        pin_in:     u8,
        reading:    u128,
    }
    
    impl Ultrasonic {
        pub fn new(pin_out: u8, pin_in: u8) -> Ultrasonic {
            Ultrasonic {
                pin_out,
                pin_in,
                reading:    0,
            }
        }
    
        pub fn get_median_reading(&mut self) -> u128 {
            let mut last_ten    = Vec::with_capacity(MEDIAN_READINGS);
            let mut pin_out     = OutputDevice::new(self.pin_out);
            let pin_in          = InputDevice::new(self.pin_in);
    
            for _ in 0..MEDIAN_READINGS {
                // send sonic
                pin_out.on();
                std::thread::sleep(Duration::from_micros(TIME_NEXT_READING));
                pin_out.off();
                
                // measure
                let check_fail      = Instant::now();
                while !pin_in.is_active() { 
                    if check_fail.elapsed().as_micros() > TIME_TO_FAIL {
                        println!("ultrasonic sensor failed.");
                        return 0;
                    }                
                }
                
                let time_start      = Instant::now();
                
                while pin_in.is_active() {}
    
                let time_elapsed    = time_start.elapsed().as_micros();
                let mut distance    = time_elapsed / DIVISOR_INCHES; 
    
                // remove obvious outliers -- those past max range of device
                if distance >= MAX_IN_INCHES { distance = self.reading }
                last_ten.push(distance);
            }
    
                self.reading = *last_ten.select_nth_unstable(MEDIAN_INDEX).1; // return median
                self.reading
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    pub trait DetectPerson {
        fn person_detected(&mut self, min_distance: u128) -> bool;
    }
    
    impl DetectPerson for ultrasonic::Ultrasonic {
        fn person_detected(&mut self, min_distance: u128) -> bool {
            self.get_median_reading() < min_distance
        }
    }
    
    #[test]
    fn test1() {
        let mut ultrasonic = ultrasonic::Ultrasonic::new(5, 6);
        assert_eq!(false, ultrasonic.person_detected(30));
    }
}


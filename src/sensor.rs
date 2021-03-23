pub mod sensor {
    use std::time::{Duration, Instant};
    
    use rust_gpiozero::*;
    use crossbeam_channel::{Sender, Receiver, bounded};
    use crossbeam_utils::thread;

    const MEDIAN_READINGS: usize    = 11;
    const MEDIAN_INDEX: usize       = 6;

    const WAIT_FOR_READING: u64     = 60;
    const WAIT_SHUTDOWN: u64        = 50;

    const DIVISOR_INCHES: u128      = 148;
    const DIVISOR_CM: u128          = 148;

    const OFF: u128                 = 9999;

    pub struct Ultrasonic {
        pin_out:    OutputDevice,
        pin_in:     InputDevice,
        reading:    u128,
        sender:     Sender<u128>,
        receiver:   Receiver<u128>,
    }

    impl Ultrasonic {
        pub fn new(pin_out: OutputDevice, pin_in: InputDevice) -> Ultrasonic {
            let (s, r)      = bounded(20);

            Ultrasonic {
                pin_out,
                pin_in,
                reading:    0,
                sender:     s,
                receiver:   r,
            }
        }

        pub fn start(&mut self) {
            let mut last_ten = Vec::with_capacity(MEDIAN_READINGS);
            
            thread::scope(|s| {
                s.spawn(|_| loop {
                    let distance = self.get_sensor_readings();
                    last_ten.push(distance);

                    if last_ten.len() == MEDIAN_READINGS {
                        let median = *last_ten
                                     .clone()
                                     .select_nth_unstable(MEDIAN_INDEX)
                                     .1;

                        self.sender.send(median);
                        last_ten.remove(0);
                    }

                    // check for msg to shutdown
                    if let Ok(is_off) = self.receiver.try_recv() {
                        println!("Received message for shutdown: {}", is_off);
                        if is_off == OFF { break }
                    }

                    // sensor requires time before each reading
                    std::thread::sleep(Duration::from_millis(WAIT_FOR_READING));
                });
            }).unwrap();

            match self.receiver.recv() {
                Ok(msg)     => println!("Reading: {}", msg),
                Err(e)      => println!("Error getting reading: {}", e),
            }
        }

        pub fn stop(&self) {
            match self.sender.send(OFF) {
                Ok(send)    => println!("Ultrasonic sensor shutting down"),
                Err(e)      => println!("Ultrasonic shutdown err: {}", e),
            }
        }

        pub fn get_reading(&self) -> u128 {
            self.reading
        }

        fn get_sensor_readings(&mut self) -> u128 {
            // send sonic
            self.pin_out.on();
            std::thread::sleep(Duration::from_micros(5));
            self.pin_out.off();
            
            // measure
            let check_fail      = Instant::now();
            while !self.pin_in.is_active() { 
                if check_fail.elapsed().as_micros() > 1700 {
                    println!("ultrasonic sensor failed.");
                    return 0;
                }                
            }
            
            let time_start      = Instant::now();
            
            while self.pin_in.is_active() {}

            let time_elapsed    = time_start.elapsed().as_micros();
            let distance        = time_elapsed / DIVISOR_INCHES; 
            // println!("Distance: {:?}", distance);
            
            distance
        }
    }
}

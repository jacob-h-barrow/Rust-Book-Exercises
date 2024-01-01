// Chapter 5 Code
//// Modeled after my Leetcode post for 1603: Designing Parking Systems

use rand::Rng;
use std::{thread, time};

struct ParkingSpots {
    car_type: i32,
    capacity: i32,
    taken: i32,
    car_type_str: String
}

impl ParkingSpots {
    fn new(car_type: i32, capacity: i32) -> Result<Self, String> {
        if capacity < 0 {
            return Err(format!("{} isn't real", capacity));
        }
        if ![1, 2, 3].contains(&car_type) {
            return Err(format!("{} isn't supported", car_type));
        }
        
        let car_type_str = match car_type {
            1 => String::from("big"),
            2 => String::from("medium"),
            3 => String::from("small"),
            _ => String::from("unknown"), // Handle any other values gracefully
        };
        
        Ok(Self {
            car_type,
            capacity,
            taken: 0,
            car_type_str
        })
    }

    fn add_car(&mut self) -> bool {
        if self.taken < self.capacity {
            self.taken += 1;
            self.display();
            true
        } else {
            false
        }
    }
    
    fn display(&self) {
        println!("{} ({}) has a capacity of {} and is currently occupied with {} cars!", self.car_type_str, self.car_type, self.capacity, self.taken);
    }
}

struct ParkingSystem {
    lot: Vec<ParkingSpots>,
}

impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        let lot = vec![
            ParkingSpots::new(1, big).unwrap(),
            ParkingSpots::new(2, medium).unwrap(),
            ParkingSpots::new(3, small).unwrap(),
        ];

        Self { lot }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        self.lot[(car_type - 1) as usize].add_car()
    }
}

fn main() {
    // Pun intended
    let mut big_lots = ParkingSystem::new(5, 2, 1);
    
    // Lets see if it fails!!! Shouldn't lol
    for _ in 0..8 {
        let random_num = rand::thread_rng().gen_range(1..=3);
        println!("{} generated!", random_num);
        
        big_lots.add_car(random_num);
        
        let one_sec = time::Duration::new(1, 0);
        thread::sleep(one_sec);
    }
}

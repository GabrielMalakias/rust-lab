struct RaceCar {
    number: i32,
}

impl Drop for RaceCar {
    fn drop(&mut self) {
        println!("Car {} finished the run", self.number);
    }
}

fn testing() {
    let car_c = RaceCar { number: 3 };
    let car_b = RaceCar { number: 2 };
}

fn main() {
    let car_a = RaceCar { number: 1 };
    testing();
}

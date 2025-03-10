trait Car {
    fn ignition(&mut self, drive: bool);
    fn turn(&mut self, angle: u8) -> u8;
    fn brake(&mut self, amount: i8) -> i8;
    fn accelerate(&mut self, new_speed: i8) -> i8;
    fn stop(&mut self);
}

#[derive(Debug)]
struct Battery {
    charge: i8,
}

#[derive(Debug)]
struct ElectricCar {
    battery: Battery,
    started: bool,
    speed: i8,
    direction: u8,
}

impl ElectricCar {
    fn get_charge_level(&self) -> i8 {
        self.battery.charge
    }
}

impl Car for ElectricCar {
    fn ignition(&mut self, drive: bool) {
        self.started = true
    }

    fn turn(&mut self, angle: u8) -> u8 {
        self.direction += angle;
        self.direction
    }

    fn brake(&mut self, amount: i8) -> i8 {
        self.speed -= amount;
        self.speed
    }

    fn accelerate(&mut self, new_speed: i8) -> i8 {
        self.speed += new_speed;
        self.speed
    }

    fn stop(&mut self) {
        self.speed = 0
    }
}

fn main() {
    let mut myCar = ElectricCar {
        battery: Battery { charge: 0 },
        started: false,
        speed: 0,
        direction: 0,
    };

    myCar.ignition(true);
    myCar.accelerate(25);
    myCar.brake(5);
    myCar.stop();

    println!("{:?}", myCar)
}

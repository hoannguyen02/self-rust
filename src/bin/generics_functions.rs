trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Checked in as pilot")
    }
    fn process(&self) {
        println!("Pilot enters the cockpit")
    }
}

struct Passenger; 
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Checked in as passenger")
    }
    fn process(&self) {
        println!("Passenger takes a seat")
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("Checked in as cargo")
    }
    fn process(&self) {
        println!("Cargo moves to storage")
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}
fn main() {
    let me = Pilot;
    let my_friend = Passenger;
    let cargo = Cargo;
    process_item(me);
    process_item(my_friend);
    process_item(cargo);
}
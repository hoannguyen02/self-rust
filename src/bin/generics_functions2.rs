
#[derive(Debug)]
enum ServicePriority {
    High, 
    Standard 
}

trait Priority {
    fn get(&self) -> ServicePriority;
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

#[derive(Debug)]
struct Vip;
impl Priority for Vip {
    fn get(&self) -> ServicePriority {
        ServicePriority::High
    }
}

fn print_guest<T>(guest: T) where T: Priority + std::fmt::Debug {
    println!("{:?} is {:?} priority", guest, guest.get());
}

fn main() {
    let guest = Guest;
    let vip = Vip;
    print_guest(guest);
    print_guest(vip);
}
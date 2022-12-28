enum Discount {
    Percent(i32),
    Flat(i32)
}

struct Ticket {
    event: String,
    price: i32
}

fn main() {
    let flat = Discount::Flat(20);
    match flat {
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(amount) => println!("{:?}", amount),
        _ => ()
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50
    };
    match concert {
        Ticket { price: 50, event} => println!("Event at 50.0 is {:?}", event),
        Ticket {
            price, ..
        } => println!("Price: {:?}", price)
    }

}
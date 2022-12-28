use std::vec;


enum Ticket {
    BackStage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::BackStage(50.0, "Hoan".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(4.0, "Thanh".to_owned()),
    ];
    for ticket in &tickets {
        match ticket {
            Ticket::BackStage(price, holder) => println!("Backstage Ticket Holder: {:?}, price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Price: {:?}", price),
            Ticket::Vip(price, holder) => println!("Vip Ticket Holder: {:?}, price: {:?}", holder, price),
        }
    }
}
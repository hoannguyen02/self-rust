struct Customer {
    age: Option<i32>,
    email: String,
    name: String,
}
fn main() {
    let mark = Customer {
        age: None,
        name: "Mark".to_owned(),
        email: "mark@example.com".to_owned()
    };
    let me = Customer {
        age: Some(32),
        name: "Hoan".to_owned(),
        email: "me@example.com".to_owned()
    };
    let customers = vec![
        mark,
        me
    ];

    for customer in &customers {
        match customer.age {
            Some(age) => println!("Customer {:?} is {:?} year old", customer.name, age),
            None => println!("Customer {:?} not provided age", customer.name),
        }
    }
}
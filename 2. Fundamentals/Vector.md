* Manage multiple same type of data
* Use for list of information
* Can add, remove, and traverse the entries 
* Examples
```
struct User  {
    name: String,
    age: i32
}

impl User {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
    fn display_name(&self) {
        println!("{:?}", self.name)
    }
    fn display_age(&self) {
        println!("{:?}", self.age)
    }
}

fn main() {
    // Numbers
    let numbers = vec![1,2,3];
    for num in &numbers {
        match num {
            2 => println!("second"),
            other => println!("{:?}", other)
        }
    }
    // Users
    let users = vec![
        User::new("Hoan".to_owned(), 32),
        User::new("Thanh".to_owned(), 30),
        User::new("Phuong".to_owned(), 28),
    ];
    for user in &users {
        user.display_name();
        user.display_age();
    }
}
```
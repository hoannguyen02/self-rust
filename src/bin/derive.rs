#[derive(Debug)]
struct User  {
    name: String,
    age: i32
}

fn main() {
    let me = User {
        name: "Hoan".to_string(),
        age: 32
    };
    println!("{:?}", me)
}
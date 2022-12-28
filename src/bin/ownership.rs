struct User  {
    name: String,
    age: i32
}

fn display_name(user: &User) {
    println!("{:?}", user.name)
}
fn display_age(user: &User) {
    println!("{:?}", user.age)
}
fn main() {
    let me = User {
        name: "Hoan".to_string(),
        age: 32
    };
    display_name(&me);
    display_age(&me);
}
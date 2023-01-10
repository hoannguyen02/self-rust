enum Color {
    Red,
    Blue,
    Green
}
fn main() {
    let some_user = Some("Jerry");
    if let Some(user) = some_user {
        println!("user={:?}", user);
    } else {
        println!("no user")
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("It's read");
    } else {
        println!("It's not red");
    }
}
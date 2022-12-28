#[derive(Debug)]
struct User {
    user_id: i32,
    name: String
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match  name.as_str() {
        "a"=> Some(1),
        "b"=> Some(2),
        "c"=> Some(3),
        _ => None
    }
}

fn main() {
    let user_name = "A";
    let user = find_user(user_name).map(|user_id| User {
        user_id,
        name: user_name.to_owned()
    });
   match user {
       Some(user) => println!("{:?}", user),
       None => println!("No user found")
   }
}
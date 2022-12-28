enum Access {
    Admin,
    Manager,
    User,
    Guest
}
fn main() {

    let access_level = Access::Guest;
    let can_access_file = match access_level  {
        Access::Admin => true,
        _ => false,
    };

    println!("{can_access_file}");

    let number = 99;
    let is_gt_100 = number > 100;
    match is_gt_100 {
        true => println!("It's big"),
        false => println!("It's small")
    };
}
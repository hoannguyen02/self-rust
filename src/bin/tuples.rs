fn coordinate() -> (i32, i32) {
    return (1, 7);
}
fn main() {
    let (x, y) = (1, 2);
    println!("{:?}, {:?}", x, y);

    let (x, y) = coordinate();
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Hoan", 32);
    println!("My name is {:?}, my age is {:?}", name, age);
}
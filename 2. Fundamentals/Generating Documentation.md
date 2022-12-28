* Using /// before functions, enums, structs
* Run command: cargo doc --open 
* Examples
```
/// Adds two numbers
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
fn main() {
    let result = sum(2, 2);
    println!("{:?}", result);
}
```
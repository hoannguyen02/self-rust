* A way to encapsulate program functionality 
* Optionally accept data
* Optionally return data
* Utilized for code organization, easier to read and maintain
* Examples
```
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
fn main() {
    let result = sum(2, 2);
    println!("{:?}", result);
}
```
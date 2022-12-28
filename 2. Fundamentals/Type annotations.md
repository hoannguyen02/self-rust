* Types are usually optional within function bodies but required for function signatures
* Occasionally required if compiler cannot infer the type
* Examples
```
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
fn main() {
    let a = 5;
    let b = 10;
    let result = sum(a, b);
    println!("{:?}", result);
}
```
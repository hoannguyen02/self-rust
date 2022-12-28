* A Type of record 
* Store data anonymously(no need to name fields)
* Can be destructured easily into variables 
* Useful to return pairs of data from functions
* Examples
```
fn main() {
    let (x, y) = (1, 2);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Hoan", 32);
    println!("My name is {:?}, my age is {:?}", name, age);
}
```
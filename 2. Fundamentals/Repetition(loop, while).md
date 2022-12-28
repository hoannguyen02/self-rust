* Loop
* While 
* Examples 
```
fn main() {
    let mut i  = 0;
    loop {
        if i == 3 {
            break;
        }
        println!("{:?}", i);
        i += 1
    }
}
```

```
fn main() {
    let mut i  = 0;
    while i <= 3 {
        println!("{:?}", i);
        i += 1
    }
}
```
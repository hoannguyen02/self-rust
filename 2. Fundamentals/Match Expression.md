* Similar to if..else but match will be checked by the complier 
* All possible options must be accounted for 
* Examples 
```
let bool = true;
match bool {
    true => println!("It's true"),
    false => println!("It's false"),
}
```

```
let random = 3;
match random {
    1 => println!("It's 1"),
    2 => println!("It's 2"),
    3 => println!("It's 3"),
    _ => println!("It's something else")
}
```
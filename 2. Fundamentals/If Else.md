* Code executed line by line
* Examples
```
let a = 99;
if a > 99 {
    println!("This is big number");
} else {
    println!("This is small number");
}
```

```
let a = 99;
if a > 99 {
    if a > 200 {
        println!("This is hug number");
    } else {
         println!("This is big number");
    }
} else {
    println!("This is small number");
}
```

```
let a = 99;
if a > 200 {
    println!("This is hug number");
} else if a > 99 {
    println!("This is big number");
} else {
    println!("This is small number");
}
```
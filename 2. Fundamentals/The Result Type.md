* Result<T, E> represent either success or failure 
    * OK(variable_name)
    * Error(variable_name)
* Useful when working with functionality that can potentially fail
* Examples
```
struct Adult {
    age: u8,
    name: String
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}
fn main() {
    let child = Adult::new(15, "A");
    match  child {
        Ok(c) => println!("{} is {} years old", c.name, c.age),
        Err(e) => println!("{e}")
    }
    let adult = Adult::new(23, "B");
    match  adult {
        Ok(a) => println!("{} is {} years old", a.name, a.age),
        Err(e) => println!("{e}")
    }   

}
```
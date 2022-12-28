* A type that contains multiple pieces of data , similar data can group together 
* Each piece of data is called a field
* Examples
```
struct GroceryItem {
    stock: i32,
    price: f64,
}

enum Flavor {
    Sweet,
    Fruity,
    Sparkling,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Fruity => println!("Fruity"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Sparkling => println!("Sparkling"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let cereal = GroceryItem {
        stock: 1,
        price: 2.99
    };
    println!("Stock: {:?}", cereal.stock);
    println!("Price: {:?}", cereal.price);

    let drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 9.0
    };
    print_drink(drink)
}
```
* Collection that stores data as key-value pairs
* Similar to definitions in a dictionary 
* Very fash to retrieve data using the key
* Examples 
```
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);


    let mut total = 0;

    for (item, quantity) in stock.iter() {
        total = total + quantity;
        let count = if quantity == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", quantity)
        };

        println!("item={:?}, stock={:?}", item, count)
    }

    println!("Total stock={:?}", total);
}
```
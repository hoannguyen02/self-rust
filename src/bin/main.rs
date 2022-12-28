
struct BankAccount {
    balance: i32,
    verified: bool,
}
fn main() {
    // Variables 
   let mut total = add(10, 15);
   let mut free_shipping = false;
   // Control flow 
   if total > 50 {
       free_shipping = true;
   }

    //    Match expressions
    //    match  free_shipping {
    //     true => total = total + 0,
    //     false => total = total + 5
    //    }
   total = match  free_shipping {
    true => total + 0,
    false => total + 5
   };

    // println!("{total}");
    println!("{:?}", total);

    //  Array 
    let items: [i32;  5] = [1, 2, 3, 4, 5];
    println!("{:?}", items);
    
    // Vector
    let mut vector_items = Vec::new();
    vector_items.push(1);
    println!("{:?}", vector_items);
    // Vectors with macro 
    let vector_items1 = vec![1, 2, 3, 4, 5];
    println!("{:?}", vector_items1);

    // Structures 
    let my_account = BankAccount {
        balance: 20,
        verified: false
    };
    println!("{:?}", my_account.balance);
    println!("{:?}", my_account.verified);

    // Ownership
    print_balance(&my_account);
    print_verified(&my_account);

    // Results
    let verification_status = is_verified(&my_account).expect("Unable to unwrap results");
    println!("{:?}",verification_status);

}


fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    return match account.verified {
        true => Ok(true),
        false => Err(false)
    };
}

fn print_balance(my_account: &BankAccount) {
     println!("{:?}", my_account.balance);
}
fn print_verified(my_account: &BankAccount) {
     println!("{:?}", my_account.verified);
}

fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

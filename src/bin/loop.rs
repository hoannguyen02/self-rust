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
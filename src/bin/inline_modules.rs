mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    
    pub fn sub(a: i32, b: i32) -> i32 {
        return a - b;
    }
}
fn main(){
    let sum = math::add(1, 1);
    println!("{sum}")
}
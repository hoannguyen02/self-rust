
fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
fn main(){
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_add() {
        assert_eq!(2, add(1, 1), "Expected value is 2");
    }
}
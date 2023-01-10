fn main(){
    let numbers = vec![1,2,3,4,5];
    let new_numbers: Vec<_> = numbers.iter().map(|num| num * 3).filter(|num|  num > &3).collect(); // collect simply add new vector;
    let take: Vec<_> = numbers.iter().take(3).collect();
    let count = numbers.iter().count();
    let last = numbers.iter().last();
    let min =numbers.iter().min();
    let max =numbers.iter().max();
}
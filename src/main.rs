use std::io;
fn main() {
    println!("Input two numbers separated with space");
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();
    let numbers: Vec<i32> = numbers[..numbers.len() - 2]
        .split(' ')
        .map(|num: &str| num.parse().unwrap())
        .collect();
    println!("Your numbers are: {} {}", numbers[0], numbers[1]);
}

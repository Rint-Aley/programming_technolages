use std::io;
use programming_technolages::get_rectangle_area;

fn main() {
    println!("Input sides of rectangle");
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();
    let numbers: Vec<i32> = numbers[..numbers.len() - 2]
        .split(' ')
        .map(|num: &str| num.parse().unwrap())
        .collect();
    println!("The area of rectangle is {}", get_rectangle_area(numbers[0], numbers[1]));
}

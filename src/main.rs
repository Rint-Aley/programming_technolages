use std::io;
fn main() {
    println!("Input radius and height of cylinder");
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();
    let numbers: Vec<i32> = numbers[..numbers.len() - 2]
        .split(' ')
        .map(|num: &str| num.parse().unwrap())
        .collect();
    println!("The voulume of cylinder is {}", get_cylinder_volume(numbers[0], numbers[1]));
}

fn get_cylinder_volume(radius: i32, height: i32) -> f32 {
    return std::f32::consts::PI * (radius.pow(2) * height) as f32;
}

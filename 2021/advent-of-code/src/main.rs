use std::fs;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let split_depths = input.split(",");
    let depths_strings: Vec<&str> = split_depths.collect();
    // let mut depths_ints: Vec<i32> = Vec::new();

    for depth in &depths_strings {
        println!("{}", depth);
        // let num: u32;
        // num = depth.parse().unwrap();
        // println!("{}", num);
        // depths_ints.push(depth.parse().expect("wrong number format"));
    }

    // for depth in &depths_ints {
    //     println!("{}", depth);
    // }

    // let numbers: Vec<i32> = input;
    //     .split_whitespace()
    //     .map(|s| s.parse().expect("parse error"))
    //     .collect();

    // for num in numbers {
    //     println!("{}", num);
    // }
}

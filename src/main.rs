use num_bigint::BigUint;
use num_traits::One;
use std::io;

fn main() {
    let mut number = String::new();

    println!("Please enter a number: ");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let mut number_split: Vec<u32> = Vec::new();

    number.trim().chars().for_each(|n| {
        number_split.push(n.to_digit(10).unwrap());
    });

    let mut persistance = 1;
    let mut m = persist(number_split);

    while m.len() > 1 {
        m = persist(m);
        persistance += 1;
    }

    println!("This number has a persistance of: {}", persistance);
}

fn persist(v: Vec<u32>) -> Vec<u32> {
    let mut result: BigUint = One::one();
    for x in v.iter() {
        result *= BigUint::from(*x);
    }

    println!("Result: {}", result);

    let digits: Vec<u32> = result
        .to_string()
        .chars()
        .map(|d| -> u32 { d.to_digit(10).unwrap() })
        .collect();

    digits
}

mod solver;

use std::io;

fn main() {
    let input = io::stdin();
    let mut nums = vec![];

    println!("Enter 5 numbers from Math.random:");

    for _ in 0..5 {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();

        let num: f64 = buf.trim().parse().expect("Not a number!");
        nums.push(num);
    }

    println!("Calculating...");
    let next = solver::predict_math_random(&nums);

    match next {
        Some(num) => println!("The next number is {}", num),
        None => println!("Couldn't figure out the next number."),
    }
}

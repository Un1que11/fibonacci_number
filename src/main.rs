use std::io;

fn main() {
    let mut n: String = String::new();

    println!("Please input a positive integer:");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u128 = n.trim().parse().expect("Please type a number");

    println!("{}", fibonacci(n));
}

fn fibonacci(n: u128) -> u128 {
    if n > 1 {
        fibonacci(n - 1) + fibonacci(n - 2)
    } else {
        n
    }
}

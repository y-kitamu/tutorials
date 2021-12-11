use std::io;

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("failed to read line");

    let n = n.trim().parse()
        .expect("Number is expected");

    let x = fibonacci(n);
    println!("n = {}, fibonacci[n] = {}", n, x);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci(n - 2) + fibonacci(n - 1)
}

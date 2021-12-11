fn five() -> u32 {
    5
}

fn main() {
    println!("Hello, world!");
    another_function(32, 64);
    println!("five = {}", five());
}

fn another_function(x: i32, y: i32) {
    println!("another function");
    println!("x = {}", x);
    println!("y = {}", y);
}

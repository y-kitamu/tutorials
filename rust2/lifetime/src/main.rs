fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

    println!("Hello, world!");
    let a = String::from("test");
    let c: &str;
    {
        let b = String::from("foo");

        c = longest(&a, &b);
        println!("longest : {}", c);
    }
}

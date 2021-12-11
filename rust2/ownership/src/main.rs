fn main() {
    println!("Hello, world!");

    const i0: usize = 0;
    const i1: usize = 1;
    let s0: &str = "string";
    let s = String::from("hello world");
    let s0: &String = &s;
    let hello = &s[0..5];

    let mut a = [1, 2, 3, 4, 5];
    let b = a;
    a[0] = 0;
}


fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

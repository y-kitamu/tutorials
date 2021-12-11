fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world");

    let length = take_ownership(&s);
    println!("length : {}", length);
    println!("{}", s);

    {
        let hello = &s[0..5];
        let world = &s[7..12];

        println!("{}", hello);
        println!("{}", world);
    }
    println!("{}", s);
}

fn take_ownership(some_string: &String) -> usize {
    some_string.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

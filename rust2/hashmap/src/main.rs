use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in scores {
        println!("{} : {}", key, value);
    }

    // for (key, value) in scores {
    //     println!("{} : {}", key, value);
    // }
}



fn main() {
    let mut s = String::new();

    let data = "Initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s4: &str = "Tets";
    let s5 = &s2[0..2];
    let s3: String = s1 + &s2[0..2];

    let s1 = String::from("hello");

    let s4 = "Здравствуйте";
    let s = &s4[0..2];
    println!("{}", s);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let mut v = vec![1, 2, 3, 4, 5];

    v.push(10);
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let mut v  = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(v) => println!("i = {}", v),
            SpreadsheetCell::Float(v) => println!("i = {}", v),
            SpreadsheetCell::Text(v) => println!("i = {}", v),
        }
    }
}

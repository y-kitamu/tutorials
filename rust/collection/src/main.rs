use std::collections::HashMap;

fn mean(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for v in vec {
        sum += v;
    }
    sum / vec.len()
}

fn median(vec: &Vec<i32>) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    v.copy(vec);
    v.sort();
    v[v.len() / 2]
}

fn mode(vec: &Vec<u32>) -> i32 {
    let map = HashMap::new();
    for v in vec {
        let mut i = map.entry(v).or_insert(0);
        *i += 1;
    }
    let mut max_key = 0;
    let mut max_value = -1;
    for (key, val) in map {
        if val > &max_value {
            max_key = *key;
            max_value = *val;
        }
    }
    max_key
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4 ,5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    let mut s = "initial contents".to_string();
    s.push_str("bar");
    println!("{}", s);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    let score = scores.get("Blue");
    println!("{:?}", score);

    println!("{:?}", scores);

    let vec = vec![3, 4, 1, 2, 5];
    let m = mean(vec);
    println!("{}", m);
    let m = median(vec);
    println!("{}", m);
    let m = mode(vec);
    println!("{}", m);
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // let a = [1, 2, 3, 4 ,5];
    // let index = 10;
    // let element = a[index];
    // println!("The valu of element is : {}", element);

    another_function(5);

    let x = five();
    println!("The value of x is : {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is : {}", x);
}

fn five() -> i32 {
    5
}

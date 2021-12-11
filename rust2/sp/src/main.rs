use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = MyBox::new(x);
    // let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(&x, y);

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

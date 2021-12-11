fn main() {
    println!("Hello, world!");

    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {}", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {}", area1(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("The area of the rectangle is {}", area2(&rect2));
    println!("{:?}", rect2);

    println!("The area of the rectangle is {}", rect2.area());

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

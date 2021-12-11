fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 64];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

}

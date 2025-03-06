fn main() {
    learn_18_slice();
}

fn learn_18_slice() {
    let array = [1,2,3,4,5,6,7,8,9,10];

    let range = &array[1..4];
    println!("{:?}", range);

    let range_from = &array[6..];
    println!("{:?}", range_from);

    let range_full = &array[..];
    println!("{:?}", range_full);

    let range_inclusive = &array[1..=4];
    println!("{:?}", range_inclusive);

    let range_to = &array[..4];
    println!("{:?}", range_to);

    let range_to_inclusive = &array[..=4];
    println!("{:?}", range_to_inclusive);

    let name = String::from("Hello, World!");
    let slice = &name[0..5];
    println!("{:?}", slice);
}

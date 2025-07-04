fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    println!("Values in the collection:");

    // Use iter() to loop through the collection without taking ownership
    for value in numbers.iter() {
        println!("{}", value);
    }

    // You can still use `numbers` here because iter() borrows, not moves
    println!("Original collection: {:?}", numbers);
}

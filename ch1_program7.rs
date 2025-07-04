fn main() {
    let person: (&str, i32, f64) = ("Alice", 30, 5.6);

    let name = person.0;
    let age = person.1;
    let height = person.2;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {} ft", height);
}

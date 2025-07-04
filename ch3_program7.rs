fn main() {
    let mut count = 0;
    let mut number = 1;

    // Loop until number reaches or exceeds 50
    while number < 50 {
        println!("Iteration {}: number = {}", count + 1, number);
        number *= 2; // Double the number each time
        count += 1;
    }

    println!("Total iterations: {}", count);
}

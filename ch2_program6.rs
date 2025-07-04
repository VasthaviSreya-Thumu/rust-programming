fn main() {
    // Label the outer loop
    'outer: for i in 1..=10 {
        // Label the inner loop
        'inner: for j in 1..=10 {
            print!("{}\t", i * j);

            // Just an example: if product is 50, break the inner loop
            if i * j == 50 {
                println!("\nBreaking inner loop at product 50");
                break 'inner;
            }
        }
        println!(); // Newline after each row

        // Example: break the outer loop if i is 5
        if i == 5 {
            println!("Breaking outer loop at i = 5");
            break 'outer;
        }
    }
}

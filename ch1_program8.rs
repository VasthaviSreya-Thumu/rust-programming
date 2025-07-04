fn main() {
    // Create an array of 10 elements
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // a. Create a slice of the 2nd and 3rd element (index 1 to 3, exclusive of end)
    let slice_a = &numbers[1..3];
    println!("a. Slice of 2nd and 3rd elements: {:?}", slice_a);

    // b. Omit the start index (start from beginning to index 5, exclusive)
    let slice_b = &numbers[..5];
    println!("b. Slice omitting start index (..5): {:?}", slice_b);

    // c. Omit the end index (start from index 5 to end)
    let slice_c = &numbers[5..];
    println!("c. Slice omitting end index (5..): {:?}", slice_c);

    // d. Omit both start and end (slice entire array)
    let slice_d = &numbers[..];
    println!("d. Full slice (..): {:?}", slice_d);
}

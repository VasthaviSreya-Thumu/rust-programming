fn main() {
    // 1. Declare a String object using the `String::from()` method
    let name = String::from("Rustacean");

    // 2. Convert a string literal to a String using `.to_string()` method
    let language_literal = "Rust";
    let language = language_literal.to_string();

    // 3. Declare another String directly using `.to_owned()` (another way)
    let version = "1.75".to_owned();

    println!("Name: {}", name);
    println!("Language: {}", language);
    println!("Version: {}", version);
}

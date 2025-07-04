fn main() {
    let x = 5;
    println!("Outer x = {}", x);

    {
        let x = 10;
        println!("Inner x = {}", x);

        {
            let x = 20;
            println!("Nested inner x = {}", x);
        }

        println!("Inner x after nested scope = {}", x);
    }

    println!("Outer x after inner scopes = {}", x);

    let x = x + 1;
    println!("Shadowed outer x = {}", x);
}

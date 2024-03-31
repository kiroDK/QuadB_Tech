fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|w| w.len())
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();

    if let Some(shortest) = shortest_word(input) {
        println!("Shortest word in '{}' is '{}'", input, shortest);
    } else {
        println!("No words found in '{}'", input);
    }
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    println!("Enter a string to reverse:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();

    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);
}

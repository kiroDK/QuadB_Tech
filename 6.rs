fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first_str = strs[0];
    let mut prefix = String::new();

    'outer: for (i, c) in first_str.chars().enumerate() {
        for word in &strs[1..] {
            if let Some(ch) = word.chars().nth(i) {
                if ch != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    println!("Enter a set of strings separated by spaces:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let words: Vec<&str> = input.split_whitespace().collect();

    let common_prefix = longest_common_prefix(&words);
    println!("Longest common prefix: {}", common_prefix);
}

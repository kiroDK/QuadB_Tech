fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    let mut input = String::new();
    println!("Enter a sorted array of integers separated by spaces:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Enter the number to find its first occurrence:");
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let target: i32 = input.trim().parse().expect("Invalid input");

    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}

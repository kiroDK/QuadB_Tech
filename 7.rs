fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).copied()
}

fn main() {
    println!("Enter an array of integers separated by spaces:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Enter the value of k:");
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Invalid input");

    if let Some(kth_smallest) = kth_smallest(&arr, k) {
        println!("The {}th smallest element is {}", k, kth_smallest);
    } else {
        println!("Invalid input or index out of bounds");
    }
}

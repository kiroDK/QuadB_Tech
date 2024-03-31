fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    if i < arr1.len() {
        merged.extend_from_slice(&arr1[i..]);
    }

    if j < arr2.len() {
        merged.extend_from_slice(&arr2[j..]);
    }

    merged
}

fn main() {
    println!("Enter the first sorted array of integers separated by spaces:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let arr1: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Enter the second sorted array of integers separated by spaces:");
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let arr2: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}

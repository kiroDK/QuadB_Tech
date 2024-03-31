fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];

    for &num in arr.iter().skip(1) {
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
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

    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}

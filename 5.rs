fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        let mid = n / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[n / 2] as f64
    }
}

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let median = find_median(&arr);
    println!("Median of the array is {}", median);
}

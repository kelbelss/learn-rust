use std::time::Instant;

fn main() {
    println!("Fibonacci with time measurements");

    // The recursive method's time will increase exponentially = O(2^n) time complexity.
    // The non-recursive method's time will increase roughly linearly with n = O(n) time complexity.

    for i in 0..=50 {
        let start_recursive = Instant::now();
        let recursive_results = recursive_fibonacci(i);
        let duration_recursive = start_recursive.elapsed();

        let start_non_recursive = Instant::now();
        let non_recursive_results = fib_non_recursive(i);
        let duration_non_recursive = start_non_recursive.elapsed();

        println!(
            "n = {}: Recursive = {} (in {:?}), Non-recursive = {} (in {:?})",
            i, recursive_results, duration_recursive, non_recursive_results, duration_non_recursive
        );
    }
}

fn recursive_fibonacci(n: u32) -> u128 {
    match n {
        0 | 1 => 1,
        _ => recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2),
    }
}

fn fib_non_recursive(n: u32) -> u128 {
    if n <= 1 {
        return 1;
    }

    let mut a = 1;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("Fibonacci with time measurements");

    // The non-recursive method's time will increase roughly linearly with n = O(n) time complexity.
    // The recursive method's time will increase exponentially = O(2^n) time complexity.
    // The memoisation method with resursion will reduce the time complexity of the recursive function from O(2^n) to O(n).

    for i in 0..=50 {
        let start_non_recursive = Instant::now();
        let non_recursive_results = fib_non_recursive(i);
        let duration_non_recursive = start_non_recursive.elapsed();

        let start_recursive = Instant::now();
        let recursive_results = recursive_fibonacci(i);
        let duration_recursive = start_recursive.elapsed();

        let mut memo = HashMap::new(); // Initialise a memoisation table
        let start_memoisation = Instant::now();
        let memoisation_results = memoisation_recursive_fib(i, &mut memo);
        let duration_memoisation = start_memoisation.elapsed();

        println!(
            "n = {}: Non-recursive = {} (in {:?}), Recursive = {} (in {:?}), Memoisation = {} (in {:?})",
            i, non_recursive_results, duration_non_recursive, recursive_results, duration_recursive, memoisation_results, duration_memoisation
        );
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

fn recursive_fibonacci(n: u32) -> u128 {
    match n {
        0 | 1 => 1,
        _ => recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2),
    }
}

fn memoisation_recursive_fib(n: u32, memo: &mut HashMap<u32, u128>) -> u128 {
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = match n {
        0 | 1 => 1,
        _ => memoisation_recursive_fib(n - 1, memo) + memoisation_recursive_fib(n - 2, memo),
    };

    memo.insert(n, result);
    result
}

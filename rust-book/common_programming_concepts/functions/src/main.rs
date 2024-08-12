fn main() {
    println!("Hello, world!");

    another_function();
    with_parameters(4);
    let y = learn_return();
    println!("{y}");
}

fn another_function() {
    println!("Another function.");
}

fn with_parameters(x: i32) {
    println!("The value of x is: {x}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

// statement - let y = 6
// espression (do not end with semicolon if wanting to return)

fn learn_return() -> i32 {
    5 // no semicolon
}

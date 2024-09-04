// Safety is the Absence of Undefined Behavior

// A foundational goal of Rust is to ensure that your programs never have undefined behavior.

fn main() {
    println!("Hello, world!");
}

// Notes

// All heap data must be owned by exactly one variable.
// Rust deallocates heap data once its owner goes out of scope.
// Ownership can be transferred by moves, which happen on assignments and function calls.
// Heap data can only be accessed through its current owner, not a previous owner.

fn main() {
    // Primitive Data Types - Scalar and Compound

    // Scalar - something that has a finite set of possibile values, following some scale
    // - Integer Types
    // - Floating-Point Types
    // - Booleans Types
    // - Character Types

    // Compound - can be constructed in a program using the programming language's primitive data types and other composite types
    // - The Tuple Type
    // - The Array Type

    // Scalar - Integer Types (whole numbers - signed can be negative)

    let int = -2; // (default is i32)
    let int: i32 = -2;

    let uint = 2; // no signs
    let uint: u32 = 2;

    // u8 is 2^8 - 1 = 255 (range 0 to 255)
    // i8 is -2^7 - 2^7 - 1 (range -128 to 127) (signed -(2^n-1) to (2^n-1) -1)

    // Integer Literals in Rust
    // Decimals - 98_222
    // Hex - 0xff
    // Octal - 0o77
    // Binary - 0b1111_0000
    // Byte (u8 only) - b'A'

    // Scalar - Floating-Point Types (f32 (single precision) and f64 (double precision))

    let floating_point: f32 = 10.9; // (default is f32)

    // Scalar - Booleans Types

    let is_it_false: bool = false; // underscore
    let is_it_true: bool = true; // false = 0 and true = 1

    // Scalar - Character Types

    let letter: char = '/'; // single quotation marks

    // Compound - The Tuple Type (immutable - can use mut keyword)

    let tup: (&str, i32) = ("Apples", 5); // get value out of tuple: destructuring or dot notation
    let (fruit, fruit_count) = tup; // destructure
    let fruit_count = tup.1; // dot

    // Compound - The Array Type (comma separated list with [brackets] not (parentheses)) all have to be the same type unlike the tuple

    let error_codes: [i32; 3] = [200, 404, 500];
    let not_found = error_codes[1];
    let byte = [0; 8]; // create array with 8 values all set to 0
}

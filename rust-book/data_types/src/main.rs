fn main() {
    // Primitive Data Types - Scalar and Compound

    // Scalar - something that has a finite set of possibile values, following some scale
    // - Integer Types
    // - Floating-Point Types
    // - Numeric Operations
    // - Booleans Types
    // - Character Types

    // Compound - can be constructed in a program using the programming language's primitive data types and other composite types
    // - The Tuple Type
    // - The Array Type

    // - Integer Types (whole numbers - signed can be negative)

    let int = -2; // (default is i32)
    let int: i32 = -2;

    let uint = 2; // no signs
    let uint: u32 = 2;

    // u8 is 2^8 - 1 = 255 (range 0 to 255)
    // i8 is -2^7 - 2^7 - 1 (range -128 to 127)

    // - Floating-Point Types (f32 (single precision) and f64 (double precision))

    let floating_point: f32 = 10.9; // (default is f32)

    // - Booleans Types

    let is_it__false: bool = false; // underscore
    let is_it_true: bool = true; // false = 0 and true = 1

    // - Character Types

    let letter: char = '/'; // single quotation marks
}

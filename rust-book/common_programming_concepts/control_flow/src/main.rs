fn main() {
    println!("Control Flow");

    // if statements - condition must be a boolean
    let number = 5;
    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    // if statements can be inside a let
    let condition = true;
    let num = if condition { 5 } else { 6 }; // true first, false second

    // loop - will keep going unless break statement
    loop {
        println! {"again"};
        break;
    }
    // can return values
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter; // return counter
        }
    };

    println!("result is {result}");

    // while loop
    let mut numb = 3;

    while numb != 0 {
        println!("{numb}!");
        numb -= 1;
    }

    println!("while loop done");

    // for loop
    let a = [10, 20, 30, 40, 50];
    // a.inter = interator for array
    for element in a.iter() {
        println!("the value is: {element}");
    }
    // can be used for a range
    for numberr in 4..7 {
        // last number is exclusive
        println!("{numberr}!");
    }
}

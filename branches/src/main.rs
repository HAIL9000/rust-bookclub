fn main() {
    //since if is an expression, we can use it on the other side of a let statement
    let condition = true;
    //if an else arms must have compatible value types, rust needs to know at compile
    //time what the value of 'number' will be
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    //In rust, if conditions must be bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    //else if example
    //once one block is true the rest are skipped
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

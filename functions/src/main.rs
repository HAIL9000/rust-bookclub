//Statements are instructions that perform some action and do not return a value. 
//Expressions evaluate to a resulting value. Let’s look at some examples.

fn main() {
    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        //Expressions do not include ending semicolons! Adding one will make it a statement
        x + 1
    };

    println!("The value of y is: {}", y);
    
    //let x = five();
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

// type annotations on parameters required
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

//functions return the last expression (no semicolon)
// -> declares the type that the return value will be
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
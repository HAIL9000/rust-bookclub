fn main() {
    //Chanting a mutable variable
    //let mut x = 5;
    //println!("the value of x is: {}", x);
    //x = 6;
    //println!("the value of x is: {}", x);

    //consts cannot be mutable or set to a runtime computed value
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
 
    //Shadowing a variable
    //"let" allows us to perform a few transformations on am immutable variable
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    //If "spaces" was mut we would get an error for trying to mutate its type
    //but shadowing with let is different than changing the value
    let spaces = "   ";
    let spaces = spaces.len();

    //Since guess could be many values, we give it a "u32" type annotation
    let guess: u32 = "42".parse().expect("Not a number!");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    //booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    //tuple with type annotation - let tup: (i32, f64, u8) = (500, 6.4, 1);

    //deconstructing tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    //accessing elements of a tuple
    let tupx: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tupx.0;
    let six_point_four = tupx.1;
    let one = tupx.2;

    //arrays in rust are fixed size!
    let a = [1, 2, 3, 4, 5];
    //array declaration with type annotation [type, size]
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5]; = let a = [3, 3, 3, 3, 3]
}

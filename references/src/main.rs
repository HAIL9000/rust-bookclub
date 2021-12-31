fn main() {
    let s1 = String::from("hello");
    //passes a reference to a string (pointer to a pointer)
    //creaitng a reference is called "borrowing"
    //you cannot change something you're borrowing because you don't own it
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //you can create mutable references
    //BUT you can only have one mutable reference to a particular thing at a time (in a scope)
    //this help prevents race conditions
    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

//accepts a reference to a string
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    //let s = "hello!";
    //creates a string on the heap that can be of variable size
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
   
    //creates two integers that both = 5
    //deep and shallow copy are the same here, all memory is on the stack
    //"copy" trait
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    //copies the pointer, length, and capacity of s1 but the pointer
    //still points to the same information on the heap, new memory on
    //the heap is not allocated
    //
    //when s1 and s1 go out of scope, rust will try to deallocate the
    //same memory causing an error. thus, s1 because invalid and a
    //"move" takes place to s2
    //
    //"drop" trait
    let s2 = s1;

    //using clone() will cause rust to do a deep copy, now both
    //variables are still valid and point to different allocate memory
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4= {}", s3, s4);

    let string = String::from("hello");  // s comes into scope

    takes_ownership(string);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let num = 5;                     // x comes into scope

    makes_copy(num);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn main() {
    // The following is not allowed, as they both point to the same memory in
    // heap, and the data gets dropped twice (double free error).
    // To ensure memory safety, instead of trying to copy the allocated memory,
    // Rust considers s1 invalid, and doesn't need to free anything when s1
    // goes out of scope.
    // Then the error message you gets implies that s1 is an invalidated
    // reference, after the data has been moved to s2. In other words, there's
    // no such thing as a "shallow copy" in Rust.
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // If you DO want to copy the heap data, you can use clone:
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // If we're dealing with stack-only data, we can simply:
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);
    // Here, `s` is invalidated.

    let x = 5;
    makes_copy(x);
    // Here, `x` is just fine.

    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // Note that by this point, s2 is invalidated, as its ownership to the data
    // is taken and given to s3.
    println!("s3 = {}", s3);

    // What if we want to let a function use a value but not take ownership?
}

fn takes_ownership(some_string: String) {
    // Here, `some_string` comes into scope and takes ownership.
    println!("{}", some_string);
} // After this point, because `some_string` drops and the backing memory is
  // freed, the variable out of this scope loses the ownership of the data
  // permanently.

fn makes_copy(some_integer: i32) {
    // On the other hand, when data is copied, nothing special happens.
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string  // `some_string` is returned and moves out to the calling
                 // function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // `a_string` is returned and moves out to the calling function
}
fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Shadowing is different then assigning values multiple times to an
    // existing variable (in fact, you wouldn't be able to do that, since
    // y is an immutable variable); instead, we're creating a new immutable
    // variable y, each time, with a new data type when necessary.
    let y = 5;
    let y = y + 1;
    let y = y + 2;
    println!("The value of y is {}", y);

    // Similarly the following is valid as a result:
    let spaces = "     ";  // string
    let spaces = spaces.len();  // int
    println!("The value of spaces is {}", spaces);

    // But this isn't okay:
    // let mut spaces = "     "; // string
    // spaces = spaces.len(); // int
    // ... as you can't assign an integer value to a string variable.

    // Tuples!
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);
    println!("The value of the first element of tup is {}", tup.0);

    // ... and arrays!
    // Note that arrays in Rust is closer to that in C, than Python's list,
    // i.e., their data types and lengths are fixed when created, and cannot
    // be changed.
    let arr1: [i32; 5] = [1, 2, 3, 4, 4];
    let arr2 = [3; 5];  // repeat 3, 5 times
    println!("The value of the second element of arr1 is {}", arr1[1]);
    println!("The value of the third element of arr2 is {}", arr2[2]);
}

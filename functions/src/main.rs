fn main() {
    if_expression();
    multiple_ifs();
    if_in_let();
    loop_better_example();
    while_example();
    while_example_2();
    for_example();
    for_example_2();
}

fn if_expression() {
    let number = 3;
    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }
}

fn multiple_ifs() {
    let number = 6;

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

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

// I think we understand what this does... let's not run this one.
// fn loop_example() {
//     loop {
//         println!("again!");
//     }
// }

fn loop_better_example() {
    let mut counter = 0;

    // loop { ... } here is an "expression", which yields a value.
    // This is a pretty nice property from an expression-oriented programming
    // language! Read more here:
    // https://en.wikipedia.org/wiki/Expression-oriented_programming_language
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_example() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn while_example_2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value at index {} is: {}", index, a[index]);
        index += 1;
    }
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn for_example_2() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
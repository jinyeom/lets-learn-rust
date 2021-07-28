use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100.
    // 1..101 can be replaced by 1..=100 (where = indicates inclusive).
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Since this was a part of the tutorial, we'll just leave this line as a
    // comment.
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Variables are immutable by default! Allow this variable to be
        // mutable by adding `mut` keyword before the variable name.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // The use of the same variable name as an existing one is okay, when
        // the goal is to convert its data type, e.g., in our case, we're
        // converting a String to a u32. This is called "shadowing".
        //
        // If parse() executes successfully, we return the number; move to the
        // next iteration of the loop otherwise.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

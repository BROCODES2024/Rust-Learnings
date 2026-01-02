use std::cmp::Ordering;
use std::io;
use rand::{Rng, rng};

fn main() {
    println!("Guess the number!");

    let secret_number = rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // _ means anything else
        };

        println!("You guessed: {guess}");

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
    //cmp is a method which compares two values
    //match is similar to switch in c++
    //why match insted of if else , cause match forces us to handle all cases
    // pattern =>(arm) code ie if this pattern matches run the "code"
    //here ordering is an enum it has three variants less,greaterm equal
    // ordering is enum just like result is an enum
    // since enums represent finite known cases so using match is best option
    // if using an enum we must handle all variants
    //we see that guess is already prensent but we again did let guess
    // helpfully Rust allows us to shadow the previous value of guess with a new one. 
    //Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
    //trim removes whitespaces and parse converts string to another type
    //loop keyword creates an infinte loop
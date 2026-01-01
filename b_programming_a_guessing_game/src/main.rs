//we need a crate for random number generation 
//Remember that a crate is a collection of Rust source code files.
//to add a crate u can add it in cargo.toml or use cargo add command
//example: cargo add rand
//The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.
//when we build cargo creates a cargo.lock file that records the exact version of each dependency used in the project
// and in future builds cargo will use the same versions recorded in cargo.lock to ensure reproducible builds.
//  To upgrade to a new major version (like 0.9), you must change Cargo.toml
// cargo update will update dependencies to the latest versions allowed by the version specifiers in Cargo.toml

use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

// Rng is a trait that defines random-number methods like gen_range

// Traits must be in scope to use their methods on a type

// thread_rng() creates a random number generator for the current thread

// gen_range(1..=100) generates a random number between 1 and 100 (inclusive)

// The random number is stored in secret_number

// Printing the secret number is only for debugging and will be removed later

// A trait defines what methods a type can have.

// Functions come from modules, methods often come from traits.
//cargo doc --open gives you local documentation 
//so you can learn which traits, methods, and functions a crate provides and how to use them.
use std::io;
// ordering is an enum that is a result of two things being compared
// it has 3 variants i.e Less,Equal,Greater
use std::cmp::Ordering;
// The Rng trait defines methods that random number generators use
use colored::*;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng gives us a random number generator
    // gen_range generates a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // stdin gives us a handle to the standard input of the current process
        // read_line will take the user's input and append it to the specified buffer i.e guess string
        // read_line returns a result type which has Ok and Err
        // if there is no error expect will return the value in Ok otherwise it will panic with the
        // provided message
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim removes whitespace from the beginning and end of a string
        // parse coverts our string to an int,by default parse doesn't know the type
        // to covert to, it uses the type annotated in the variable as a hint
        // redeclaring this variable here with the same name and different type is called shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // this is an alternative to the above line but the problem with this is our program
        // will panic when a user types something that is cannot be converted into an unsigned integer
        // instead we want to prompt the user to guess a number again
        // let guess: u32 = match guess.trim().parse().expect("Please type a number");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
    }
}

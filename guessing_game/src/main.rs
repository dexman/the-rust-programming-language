// Tell rust that `rand` is an external dependency.
// This also does the equivalent of `use`.
extern crate rand;

// Bring the std::io library into scope.
use std::io;

// Bring std::cmp::Ordering into scope.
use std::cmp::Ordering;

// Rng trait defines methods that the random number generators use.
// We must `use` is so that `gen_range` is visible on `thread_rng()`.
use rand::Rng;

// fn main() is the entry point to a program.
fn main() {
    // Print a message to the screen.
    println!("Guess the number!");

    // Generate a number in [1, 100] inclusive.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // Print a message to the screen.
        println!("Please input your guess.");

        // Create a new mutable variable named `guess` and bind it to a
        // `String` instance.
        //
        // `::new` indicates `new` is an associated function of `String`.
        let mut guess = String::new();

        // Read a line of text from standard input and store it in `guess`.
        // * The `stdin` function returns an `Stdin` instance.
        // * The `read_line` method gets a line of text.
        // * & indicates guess is a reference, so that the string does not
        //   need to be copied.
        // * Use expect to capture the `Result` returned from `read_line` and
        //   fail the program if `read_line` failed
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Parse an integer from the `guess` string or fail.
        // This guess shadows the old guess.
        // `: u32` is a type annotation for the variable and chooses which
        // parse will be used.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Format and print a message to the screen.
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");

                // Exit `loop`
                break;
            }
        }
    }

    println!("The secret number is: {}", secret_number);

}

use std::io; // to obtain user input and print output
use std::cmp::Ordering;
use rand::Rng;
fn main() { // entry point of program the "main func"
    println!("Guess the number!"); // marco to print 
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
    
// to make variable -> Let apple = 12; (immutable)
// to make variable mutable -> Let mut apple = 12;
// variables, references are immutable by default
        let mut guess = String::new(); //a function that returns a new instance of a String

        io::stdin() //stdin function from the io module, which will allow us to handle user input:
//  The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
            .read_line(&mut guess) //The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.
            .expect("Failed to read line");
// is same as ...
// io::stdin().read_line(&mut guess).expect("Failed to read line");
//      let guess: u32 = guess.trim().parse().expect("Please type your number!"); //trim removes the spaves and new line character
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue, // if error throws out, it didn't kill the program
        };
    // whereas parse method parses the string into some kind of number
    // parse method will only work on character that logically converted into the numbers 
    // if parse return an err result variant because it couldn't create a number from the string,the except call will crash the game and print the message we given to it
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;          
            }
        }

        // println!("The secret number is: {}", secret_number); 
    }
}

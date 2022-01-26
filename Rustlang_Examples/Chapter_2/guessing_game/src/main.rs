use std::io; // Used to import input/output functionallity from 
             // the standard library.
use rand::Rng;
use std::cmp::Ordering;

//let apples = 5; // Immutable, object whose value can't change.
//let mut bananas = 5; // Mutable, object whose value can change.

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() // We call the stdin function from the io module
                    // which will allow us to handle the user input  

            .read_line(&mut guess) // & references are immutable by default
                                // hence why you need &mut guess

            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
            //u32 can only contain numberical data

            //to further refine our readline to accept a non number
            let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => { println!("Please enter a number between 0 and 100.");
                    continue;}
            };

        println!("You guessed: {}", guess,);

        match guess.cmp(&secret_number) { // compares guess and the secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //to stop the infinite loop
            }
        }
    }
}

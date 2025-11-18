use std::io; 
use rand::Rng; 
use std::cmp::Ordering; 

fn main() {
    println!("Guess the number!");
    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new(); 

        println!("Please input your guess"); 

        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(result) => {
                result 
            }
            Err(_) => {
                continue; 
            }
        }; 

        match guess.cmp(&num) {
            Ordering::Less => println!("Too less!"),
            Ordering::Equal => { println!("You got it!"); break; },
            Ordering::Greater => {
                 println!("Too much");
            },
        }

        println!("You guessed {guess}"); 
    }
}

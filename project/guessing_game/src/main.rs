use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        {
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            if guess.trim().to_uppercase().eq_ignore_ascii_case("Q") {
                println!("You quit the game!");
                break;
            }
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input number !");
                    continue;
                }
            };

            println!("Your guessed number:{guess}");

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
}

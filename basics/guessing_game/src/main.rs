use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let mut counter: u32 = 1 ;
    loop{
        let secret_number  = rand::thread_rng().gen_range(1..=100);
    
        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to readline");

            let guess: u32 = match guess.trim().parse() { Ok(num) => num, Err(_) => continue,};
            println!("You guessed: {guess}");

            match guess.cmp(&secret_number){
                Ordering::Less => println!("Too small!"),
                Ordering::Greater=> println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }

            }
        }

        println!("The Secret Number was {secret_number}");
        counter += 1;
        match counter.cmp(&3) {
            Ordering::Less => println!("\n Next Game {counter}!\n"),
            Ordering::Greater => {
                println!("\n HOW!\n");
                break;
            },
            Ordering::Equal => {
                println!("All games done");
                break;
            } }
    }
}


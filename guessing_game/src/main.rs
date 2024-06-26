use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    loop {
        println!("Make a guess!");

        let mut guess =  String::new();

        let secret = rand::thread_rng().gen_range(1..=100);

        io::stdin()
        .read_line(&mut guess)
        .expect("No message recieved!");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("YOU WIN!!");
                break;
            },
            Ordering::Less => println!("Too Low!")
        }

        println!("You guessed {}", guess);
        println!("The secret was {}", secret);
    }
}

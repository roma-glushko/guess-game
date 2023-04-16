use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("🔮 Guess Game");

    let mut num_tries: u16 = 0;
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        num_tries = num_tries + 1;

        println!("🎩Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("A number was expected!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small ⬆️"),
            Ordering::Greater => println!("Too big ⬇️"),
            Ordering::Equal => {
                println!("You won in {num_tries} tries 🎉");

                break;
            }
        }
    }
}

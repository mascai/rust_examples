/*
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
*/


use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret = rand::thread_rng().gen_range(0..=100);
    println!("Input your guess: ");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = guess.trim().parse()
            .expect("Failed to convert string to number");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win. ecret number is {secret}");
                break;
            }
        }
    }
}

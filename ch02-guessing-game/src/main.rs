use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("猜数字游戏!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("秘密数字是: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","太小!".red()),
            Ordering::Greater => println!("{}","太大!".red()),
            Ordering::Equal => {
                println!("{}","你赢了!".green());
                break;
            }
        }
    }
}


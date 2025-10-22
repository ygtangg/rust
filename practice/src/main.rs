use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    // Simple guessing game: 
    //  randomly generate number
    //  taking user input
    let correct = rand::rng().random_range(1..=10);
    // println!("correct: {correct}");
    println!("hey, guess a number 1-10?");

    // loops until guessed correct number8
    loop {
      let mut guess = String::new();
      io::stdin()
        .read_line(&mut guess)
        .expect("error reading input");

      // type caset: convert input type to integer
      // using match for error handling
      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(e) => {
          println!("error with parse, try again. {e}");
          continue;
        }
      };

      println!("you guessed: {guess}"); // trim removes new line character before and after name

      // if expression: returns value 
      /*
      let message = if correct < guess {
        "too high :("
      } else if correct > guess {
        "too low :("
      } else {
        "correct ;)"
      };
      println!("{message}");
      */

      // match
      match guess.cmp(&correct) {
        Ordering::Greater => println!("too high :("),
        Ordering::Less => println!("too low :("),
        Ordering::Equal => {
          println!("correct ;)");
          break;
        },
      };

    }
}
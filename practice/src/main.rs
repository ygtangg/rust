use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    /*
      Simple guessing game: 
        - randomly generate one number
        - taking user input
      
      Enhanced version:
        - randomly generate multiple number
        - user wins when correctly guessing all
     */
    let err_read_msg = "error reading input";

    // Generates correct answers
    let mut how_many = String::new();
    println!("hey, how many random numbers would you like to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect(err_read_msg);
    let num_guesses: u8 = how_many.trim().parse().expect(err_read_msg);

    let mut correct = Vec::new();
    for _ in 0..num_guesses {
      correct.push(rand::rng().random_range(1..=10));
    }    

    // Main game loop
    let mut guesses_made = 0;
    println!("hey, guess a number 1-10?");

    // loops until guessed correct number8
    while guesses_made < num_guesses {
      let mut guess = String::new();
      io::stdin()
        .read_line(&mut guess)
        .expect(err_read_msg);

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
      match guess.cmp(&correct[guesses_made as usize]) {
        Ordering::Greater => println!("too high :("),
        Ordering::Less => println!("too low :("),
        Ordering::Equal => {
          println!("correct ;)");
          guesses_made += 1;
          if guesses_made < num_guesses {
              println!("try guessing another number now!");
          }
        },
      };
    }
    println!("congrats, thanks for playing! the correct answers are");
    for item in correct {
      println!("{item}");
    }
}
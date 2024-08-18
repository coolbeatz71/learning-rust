use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;


pub(crate) fn get_guessing() {
  println!("🎮 GUESS THE NUMBER GAME 🎮");
  println!("===========================");

  let secret_number: u32 = rand::thread_rng().gen_range(1..101) as u32;
  println!("Guessing number: {}", secret_number);  
  
  loop {
    let mut guess: String = String::new();  
    println!("{}", "Please input your guess.".bright_yellow().italic());  
    
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("{}", "Input must be a number!".red());
        continue;
      },
    };
    
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("{}", "Number too small!".red()),
      Ordering::Greater => println!("{}", "Number too big!".red()),
      Ordering::Equal => {
        println!("{}", "Bingo! You win!".green().bold());
        break;
      },
    }
  }
}


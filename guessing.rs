use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub(crate) fn get_guessing() {
  println!("Guess the number");
  println!("Please input your guess.");  
  
  let mut guess: String = String::new();  
  let secret_number: u32 = rand::thread_rng().gen_range(1..101) as u32;
  println!("Guessing number: {}", secret_number);  
  
  io::stdin().read_line(&mut guess).expect("Failed to read line");
  
  let guess: u32 = guess.trim().parse().expect("Please type a number!");
  
  println!("You guessed: {}", guess);

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Number too small!"),
    Ordering::Greater => println!("Number too big!"),
    Ordering::Equal => println!("Bingo! You win!"),
  }
}


use clap::Parser;
use rand::prelude::*;

#[derive(Parser,Debug)]
#[command(version,about,long_about = None)]

struct Args{
    #[arg(short,long)]
    number:i32
}

fn main(){

  let mut rng = rand::thread_rng();
 println!("Welcome to the guessing game!");
  let args = Args::parse();
  let number = args.number;
  let random_num = rng.random_range(1..=100);

  if number == random_num{
    println!("You guessed the number!");
  }else if  number > random_num{
    println!("Your guess is too high!");
  }else{
    println!("Your guess is too low!");
  }

  println!("The random number was: {}", random_num);

  
}

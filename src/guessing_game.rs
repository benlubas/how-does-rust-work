use std::io; 
use rand; 
use rand::Rng; 
use std::cmp::Ordering; 

pub fn gg() {
  let num:u8 = rand::thread_rng().gen_range(1, 101);  

  println!("Num: {}", num);

  loop{
    println!("Make a guess: ");
    let mut guess = String::new(); 
    io::stdin().read_line(&mut guess).expect("Failed to read line"); 

    let guess: u8 = match guess.trim().parse() {
      Ok(n) => n, 
      Err(_) => {println!("numbers only"); continue},
    };

    // a Rust way of doing it 
    match guess.cmp(&num) {
      Ordering::Greater => println!("Too high"),
      Ordering::Equal => {
        println!("good job"); 
        break; 
      }, 
      Ordering::Less => println!("Too low"), 
    }

    // More traditional way of doing it 
    // both work perfectly fine. 
    
    // if guess > num {
    //   println!("Too high"); 
    // } else if guess < num {
    //   println!("Too low"); 
    // } else if guess == num {
    //   println!("Good job");
    //   break; 
    // }
  }

}
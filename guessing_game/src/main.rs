use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number?");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop {    
        println!("Please input your guess.");

        //mutable var that is bound to a new, empty instance of string
        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
       
        //trim() - rm whitespace
        //parse() - parses string and makes it what you want, eg u32 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);


        //Allows branch pattern matching kinda like haskell
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }  
    } 
}
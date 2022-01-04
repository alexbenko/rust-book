use std::io; // input/output package
use std::cmp::Ordering; //used to compare two values and returns 3 possible enums (see below)
use rand::Rng;

//In Rust, variables are immutable by default
fn main() {
    println!("Guess a number !");
    let secret_number = rand::thread_rng().gen_range(1..101); // creates an i32 var that cannot be changed

    //The & indicates that this argument is a reference,
    //which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
    // references are immutable by default as well
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // creates an empty string variable that can be changed
        io::stdin().read_line(&mut guess)
            .expect("Failed To Read line...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your Number is smaller!"),
            Ordering::Greater => println!("Your number is greater"),
            Ordering::Equal => {
                println!("Congrats You Win!");
                break;
            },
        }
    }
}

use std::io; //just like in c, we can call the std library from the header of our file
             //this is called prelude
use rand::Rng;
use std::cmp::Ordering; //module to compare guess

fn main() {
    println!("Guess the number!");

    let _secret_num = rand::thread_rng().gen_range(1..=50); // specifying num between 1 and 100

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); //new str obj set to undefined, no data defined yet. mut = not
                                       //const.

        io::stdin() //notice the formatting, it could have been written in one line but like this, it
            //looks better
            .read_line(&mut guess) //variable format we're expecting
            .expect("Failed to load read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, // if type is num we return as usual 
            Err(_) => continue, // if not we continue iterating
        };                                                       

        println!("You guessed: {guess}");

        match guess.cmp(&_secret_num) {
            //comparing guess var with arg
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => { //code block upon condition
                println!("You Guessed!!");
                break;
            },
        }
    }
}

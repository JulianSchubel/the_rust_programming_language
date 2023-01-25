use std::io; //#include equivalent.
use rand::Rng; //Bring the Rng methods into scope.
use std::cmp::Ordering;

fn main() {
	println!("Guess the number!");

	let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        /* Variables are immutable by defaut in Rust. Therefore must explicitly state the variable is mutable with "mut" */
        let mut guess = String::new();
        
        /* References are also immutable by default in Rust, hence we need to explicitly state "&mut guess" instead of "&guess" */
        
        /*
            .read_line returns a io::Result.
            Result types are enumerations.
            Variants of Result are Ok or Err.
            If io::Result is Err, .expect returns the argument (Error message) passed to it.
            If io::Result is Ok, .expect will return the value Ok is holding.
        */

        /* Method calls are split into newlines as a formatting standard */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        //println!("The random number was: {}", secret_number);

        /* cmp (compare) can be called on anything that can be compared */	
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }

}

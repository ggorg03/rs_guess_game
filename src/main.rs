use std::io::{stdin, stdout, Write};
use std::num::IntErrorKind;

use rand::{thread_rng, Rng};

fn get_guess() -> u32 {
    let mut guess = String::new();

    loop {
        guess.clear(); // clear string

        print!("What's your guess? ");
        stdout().flush().unwrap();

        stdin().read_line(&mut guess).unwrap();

        match guess.trim().parse() {
            Ok(val) => return val,
            Err(error) => match error.kind() {
                IntErrorKind::InvalidDigit => {
                    println!("Your guess must be a positive integer");
                },
                IntErrorKind::Empty => {
                    println!("Please, make a guess");
                }
                _ => {
                    println!("{}", error)
                }, // default error
            },
        };
    }
}

fn generate_secret_number() -> u32 {
    let mut rgn = thread_rng();

    rgn.gen_range(0..101)
}

fn print_score(tries: u32, record: u32) {
    let record_msg = if record == u32::MAX {
        format!("x")
    } else { 
        format!("{}", record)
    };
    
    println!("current try: {tries} | record: {}", record_msg);
}

fn wanna_continue(record: u32) -> bool {
    let mut choice = String::new();
    choice.clear(); // clear string

    println!("Wanna a new game? [y/N]");
    println!("The current record still is {}", record);

    stdin().read_line(&mut choice).unwrap();

    choice.trim() == "y"
}

fn main() {
    let mut record = u32::MAX;

    loop {
        let mut tries: u32 = 1;
        let secret_number = generate_secret_number();
        
        loop {
            print_score(tries, record);
            let guess = get_guess();

            if secret_number < guess {
                println!("No, your guess is bigger");
            }
            else if secret_number > guess {
                println!("No, your guess is lower");
            }
            else {
                if record > tries {
                    println!("Congrats, It's a new record!!");
                    record = tries;
                }
                else {
                    println!("Congrats, you got it!!");
                }
                break;
            }
            tries += 1;
        }
        
        if wanna_continue(record) { continue; }
        break;
    }
}

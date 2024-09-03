extern crate guess_game;
use guess_game::{ask_for_guess, compare, gen_rand_num};
fn main() {
    loop {
        println!("Hello, I just thought of a number between 1-10. Guess the number. I will tell you if you are right: (Press 0 to quit at anytime)");

        let computer_num = gen_rand_num();
        let guess = ask_for_guess().unwrap();
        let attempt_to_parse_guess = guess.trim().parse::<i32>();
        let _parsed_guess = match attempt_to_parse_guess {
            Ok(num) => {
                if num > 10 || num < 0 {
                    println!("Invalid input");
                    continue;
                } else {
                    num
                }
            }
            Err(e) => {
                println!("Invalid input: {}", e);
                continue;
            }
        };
        let grade = compare(computer_num, &guess);
        if guess.trim() == "0".to_owned() {
            println!("See ya!");
            break;
        }

        println!(
            "You guessed {} , which is {} my number. The number I thought of is: {}\n",
            guess, grade, computer_num
        );
    }
}

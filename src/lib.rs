extern crate rand;
use rand::Rng;
use std::{cmp::Ordering, io};

pub fn gen_rand_num() -> i32 {
    let computer_num = rand::thread_rng().gen_range(1, 11);
    computer_num
}

pub fn ask_for_guess() -> io::Result<String> {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;
    Ok(guess)
}
pub fn compare(comp_num: i32, guess: &String) -> String {
    let guess_int = guess.trim().parse::<i32>().unwrap();
    let g = match guess_int.cmp(&comp_num) {
        Ordering::Equal => "equal to".to_owned(),
        Ordering::Greater => "greater than".to_owned(),
        Ordering::Less => "lesser than".to_owned(),
    };
    g
}

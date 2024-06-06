use rand::Rng;
use std::cmp::Ordering;
use std::{io, num::ParseIntError};

fn input_int(prompt: String) -> Result<i32, ParseIntError> {
    println!("{}", prompt);
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");

    amount.trim().parse()
}

fn bet(amount: i32) -> i32 {
    let win_factor = rand::thread_rng().gen_range(25..=150);
    (amount as f64 * (win_factor as f64 / 100.0)) as i32
}

struct Player {
    name: String,
    money: i32,
}

impl Player {
    // Constructor
    fn new(name: &str, money: i32) -> Player {
        Player {
            name: name.to_string(),
            money,
        }
    }

    // Mutator
    fn bet(&mut self, amount: i32) -> i32 {
        let amount_gained = bet(amount);
        if amount_gained > 0 {
            self.money = (self.money + amount) + amount_gained;
        } else if amount_gained < 0 {
            self.money = (self.money - amount) - amount_gained;
        }
        -amount + amount_gained
    }
}

fn main() {
    let mut player = Player::new("Collin", 5_000);

    while player.money >= 0 {
        let amount = input_int(format!(
            "You have ${}. How much do you want to bet?",
            player.money
        ))
        .unwrap();
        let amount_gained = player.bet(amount);

        match amount_gained.cmp(&0) {
            Ordering::Less => println!("You lost ${}", -amount_gained),
            Ordering::Greater => println!("You gained ${}", amount_gained),
            Ordering::Equal => print!("Your money didn't change"),
        }
    }
    println!("{} lost the game.", player.name);
}

pub mod guessing_game {
    use std::{
        io::{self, Write},
        thread, time,
    };

    use rand::Rng;

    // Global module constants
    const CHANCES: u8 = 10;

    fn number_generator() -> u8 {
        let num: u8 = rand::thread_rng().gen_range(0..100);
        num
    }

    fn display_banner() {
        print!("\x1B[2J\x1B[1;1H");
        println!("You are now playing...");
        println!(
            r"
   ________  __________________    ________  ________   _   ____  ____  _______  __________ 
  / ____/ / / / ____/ ___/ ___/   /_  __/ / / / ____/  / | / / / / /  |/  / __ )/ ____/ __ \
 / / __/ / / / __/  \__ \\__ \     / / / /_/ / __/    /  |/ / / / / /|_/ / __  / __/ / /_/ /
/ /_/ / /_/ / /___ ___/ /__/ /    / / / __  / /___   / /|  / /_/ / /  / / /_/ / /___/ _, _/ 
\____/\____/_____//____/____/    /_/ /_/ /_/_____/  /_/ |_/\____/_/  /_/_____/_____/_/ |_|"
        );
        print!("\nDo you want to play the game (Yes/No): ");
        io::stdout().flush().expect("failed to flush");
    }

    fn prompt_play_again() -> bool {
        loop {
            display_banner();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => {
                    println!("\nerror: invalid input, should be yes or no values");
                    thread::sleep(time::Duration::from_secs(2));
                }
            }
        }
    }

    fn run_game(rand_number: u8) -> bool {
        let mut chances_used = 0;
        while chances_used < CHANCES {
            println!("Chance {} of {}:", chances_used + 1, CHANCES);
            print!("Enter your guess between 0 to 100: ");
            io::stdout().flush().expect("failed to flush");

            let mut input = String::from("");

            io::stdin()
                .read_line(&mut input)
                .expect("error: failed to read from stdin");

            let guess: u8 = match input.trim().parse() {
                Ok(n) => {
                    chances_used += 1;
                    n
                }
                Err(_) => {
                    println!("guess should be of type integer.");
                    continue;
                }
            };

            if guess > rand_number {
                println!("The number is less than {}", guess);
            } else if guess < rand_number {
                println!("The number is higher than {}", guess);
            } else {
                return true;
            }
        }
        false
    }

    pub fn play() {
        while prompt_play_again() {
            // Business Logic
            let rand_number = number_generator();
            let won = run_game(rand_number);

            if won {
                println!("\nHURRAY!! You got the correct answer {}", rand_number);
            } else {
                println!("\nYou lost :( Better Luck next time");
            }

            // Play Again
            println!("\nPress Enter to continue");
            let mut _enter = String::new();
            io::stdin().read_line(&mut _enter).ok();
        }
    }
}

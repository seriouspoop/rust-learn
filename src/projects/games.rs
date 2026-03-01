pub mod guessing_game {
    use std::{
        io::{self, Write},
        thread, time,
    };

    use rand::Rng;

    // Global module constants
    const CHANCES: u8 = 5;

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

    pub fn play() {
        loop {
            display_banner();

            // Get user input
            let mut is_game_on = String::from("");
            io::stdin()
                .read_line(&mut is_game_on)
                .expect("Failed to read line");
            let is_game_on = is_game_on.trim();

            match is_game_on {
                "y" | "Yes" | "YES" | "Y" | "yes" => {
                    println!("\nWelcome to the Game!!");
                }
                "n" | "No" | "NO" | "N" => {
                    println!("\nSee you again 👋");
                    break;
                }
                _ => {
                    println!("\nerror: invalid input, try again");
                    let sleep_dur = time::Duration::from_secs(2);
                    thread::sleep(sleep_dur);
                    continue;
                }
            }

            // Business Logic
            let rand_number = number_generator();
            let mut won = false;

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
                    won = true;
                    break;
                }
            }

            if won {
                println!("\nHURRAY!! You go the correct answer {}", rand_number);
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

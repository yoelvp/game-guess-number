use rand::Rng;
use std::io::{self, Write};
use termion::color;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=20);
    let mut number_attemps: i8 = 0;

    while number_attemps <= 3 {
        println!("Enter a number less than 20.");
        print!("{}# > {}", color::Fg(color::Green), color::Fg(color::Reset));
        io::stdout().flush().unwrap();

        let mut number_entered = String::new();

        io::stdin()
            .read_line(&mut number_entered)
            .expect("Don't read your number enter...");

        let number_entered: u8 = number_entered.trim().parse().unwrap();

        if number_entered == secret_number {
            println!(
                "{}Great¡{}, you got it right, the number entered is {}{}{} and the random number was {}{}{}.",
                 color::Fg(color::Green),
                 color::Fg(color::Reset),
                 color::Fg(color::Yellow),
                 number_entered,
                 color::Fg(color::Reset),
                 color::Fg(color::Yellow),
                 secret_number,
                 color::Fg(color::Reset)
            );
            return;
        }

        if number_entered < secret_number {
            println!(
                "The number entered {}'{}'{} is less than number generated.",
                color::Fg(color::Green),
                number_entered,
                color::Fg(color::Reset),
            )
        } else {
            println!(
                "The number entered {}'{}'{}is greater than number generated.",
                color::Fg(color::Green),
                number_entered,
                color::Fg(color::Reset)
            )
        }

        if number_attemps > 3 {
            println!(
                "{}You exceeded the limit of attemps.{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
            );
            return;
        }

        number_attemps += 1
    }
}

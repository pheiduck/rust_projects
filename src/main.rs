use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Rate die Zahl!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Bitte gib deine Schätzung ein.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fehler beim Lesen der Zeile");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Du hast geschätzt: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Zu klein!"),
            Ordering::Greater => println!("Zu groß!"),
            Ordering::Equal => {
                println!("Du hast gewonnen!");
                break;
            }
        }
    }
}


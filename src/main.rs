use core::num;
use std::{collections::HashMap, io};

struct Goat {
    pub name: String,
    pub power_level: u32,
    pub is_grumpy: bool,
}

impl Goat {
    /// Logs the most important facts about a goat.
    fn log(&self) {
        let grumpy_label = if self.is_grumpy {
            "is grumpy"
        } else {
            "is super happy"
        };
        let power_label = match self.power_level {
            0..=10 => "not very powerful",
            11..=99 => "pretty powerful",
            100..=199 => "SUPER powerful",
            200.. => "EXTREEEMELY powerful",
            // for edge cases _ => "this other option",G
        };
        println!(
            "{0} has a power level of {1}, which is {power_label}, and {grumpy_label}.",
            self.name, self.power_level
        )
    }
}

/// Prints a number of goats.
fn main() {
    let mut goat_count: u8 = 5;
    goat_count += 7;

    // Math is done...
    println!("We have {goat_count} goats!");

    let mut goat_map = HashMap::from([
        (
            "gruf".to_string(),
            Goat {
                name: "Gruf".to_string(),
                power_level: 999,
                is_grumpy: true,
            },
        ),
        (
            "fawn".to_string(),
            Goat {
                name: "Fawn".to_string(),
                power_level: 2,
                is_grumpy: false,
            },
        ),
        (
            "billy".to_string(),
            Goat {
                name: "Billy".to_string(),
                power_level: 32,
                is_grumpy: true,
            },
        ),
        (
            "george".to_string(),
            Goat {
                name: "George".to_string(),
                power_level: 117,
                is_grumpy: false,
            },
        ),
    ]);

    // for goat in goats {
    //     if !goat.is_grumpy {
    //         goat.log();
    //     }
    // }

    // Takes a user-supplied name, then tells them it's great.  Is it?  Who knows...

    loop {
        println!("\nPlease name a goat.");
        let mut input_text_butter = String::new();
        io::stdin()
            .read_line(&mut input_text_butter)
            .expect("I didn't get that.");

        // Prevents "enter" for goat name from making the output appearing on more than one line
        let trimmed_name: String = input_text_butter.trim().chars().collect();
        println!("{trimmed_name} is a great name for a goat!");
        let lowercased_goat_name = trimmed_name.to_lowercase();
        let goat_lookup_result = goat_map.get(lowercased_goat_name.as_str());
        match goat_lookup_result {
            Some(goat) => {
                println!("We know a goat named {trimmed_name}!");
                goat.log();
            }
            None => {
                println!("We don't know a goat by that name.");
                let name = trimmed_name;
                let power_level: u32;

                loop {
                    input_text_butter.clear();
                    println!("What is their power level?");

                    io::stdin()
                        .read_line(&mut input_text_butter)
                        .expect("I didn't get that.");

                    let trimmed_power_level = input_text_butter.trim();

                    // Please enjoy the turbo fish.
                    let number_parse_status = trimmed_power_level.parse::<u32>();
                    match number_parse_status {
                        Ok(x) => {
                            power_level = x;
                            break;
                        }
                        Err(_) => println!("I couldn't parse your bogus number."),
                    }
                }

                let is_grumpy: bool;
                loop {
                    input_text_butter.clear();
                    println!("Is the goat grumpy?!?");
                    io::stdin()
                        .read_line(&mut input_text_butter)
                        .expect("Something made this grumpy.");

                    let trimmed_grumpy = input_text_butter.trim().to_lowercase();
                    match trimmed_grumpy.as_str() {
                        "y" | "yes" | "hell yeah" | "true" => {
                            is_grumpy = true;
                            break;
                        }
                        "n" | "no" | "fuck nah" | "false" => {
                            is_grumpy = false;
                            break;
                        }
                        _ => println!("You are trippin'.  Supply a better answer (think yes/no.)"),
                    };
                }
                let new_goat = Goat {
                    name,
                    power_level,
                    is_grumpy,
                };
                println!("This is a good goat!  I will add it to the database.");
                new_goat.log();
                goat_map.insert(lowercased_goat_name, new_goat);
            }
        }
    }
}

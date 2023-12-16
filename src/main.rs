use chrono::prelude::*;
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
        let power_label = self.get_power_level_label();
        println!(
            "{0} has a power level of {1}, which is {power_label}, and {grumpy_label}.",
            self.name, self.power_level
        )
    }

    fn get_power_level_label(&self) -> &str {
        match self.power_level {
            0..=10 => "not very powerful",
            11..=99 => "pretty powerful",
            100..=199 => "SUPER powerful",
            200..=299 => "super duper powerful",
            300.. => "EXTREEEMELY powerful",
            // for edge cases _ => "this other option",
        }
    }
}

fn main() {
    // Prints a number of goats.
    // let mut goat_count: u8 = 5;
    // goat_count += 7;
    // println!("We have {goat_count} goats!");

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

    let mut sloths: Vec<String> = vec![
        "Slowy".to_string(),
        "Roberto".to_string(),
        "Speedy".to_string(),
        "Paul".to_string(),
    ];

    loop {
        println!("\nChoose an option: \n");
        println!("\t1: What time is it?");
        println!("\t2: Look at one goat.");
        println!("\t3: Log all goats!");
        println!("\t4: List sloths.");
        println!("\t5: Add sloths.");
        println!("\t6: Be a quitter.");

        let mut menu_choice = String::new();
        let read_result = io::stdin().read_line(&mut menu_choice);

        match read_result {
            Ok(_) => {
                menu_choice = menu_choice.trim().chars().collect();
            }
            Err(_) => (),
        }

        match menu_choice.as_str() {
            // Adding time, still needs to be formatted
            "1" => println!("\nIt is currently {:?}", Local::now()),
            "2" => goat_branch(&mut goat_map),
            "3" => log_all_goats(&goat_map),
            "4" => list_sloths(&sloths),
            "5" => add_sloth(&mut sloths),
            "6" => break,
            _ => println!("Invalid choice.  Pick a number 1-6."),
        }
    }
}

fn add_sloth(sloths: &mut Vec<String>) {
    let sloth_name = get_string_from_stdin("Please name a sloth.".to_string());
    println!("{sloth_name} is a great name for a sloth!");

    sloths.push(sloth_name);
}

fn list_sloths(sloths: &Vec<String>) {
    println!("\nLogging last 5 sloths!!\n");
    let end_index = sloths.len();
    // let start_index = (end_index - 5).max(0);
    let start_index = end_index.saturating_sub(5);
    let sloth_range = start_index..end_index;
    for sloth in sloths[sloth_range].iter() {
        println!("{sloth}");
    }
}

fn log_all_goats(goat_map: &HashMap<String, Goat>) {
    println!("\nLogging all goats!!\n");

    // More syntax sugar w/ destructuring - just look at 2nd thing
    for (_, goat) in goat_map {
        goat.log();
    }
    // Another way of doing this:
    // for string_goat_tuple in goat_map {
    //     let goat = string_goat_tuple.1;
    //     goat.log();
    // }
}

fn goat_branch(goat_map: &mut HashMap<String, Goat>) {
    let trimmed_name = get_goat_name();
    // Takes a user-supplied name, then tells them it's great.  Is it?  Who knows...

    let lowercased_goat_name = trimmed_name.to_lowercase();
    let goat_lookup_result = goat_map.get(lowercased_goat_name.as_str());
    match goat_lookup_result {
        Some(goat) => {
            println!("\nWe know a goat named {trimmed_name}!");
            goat.log();
        }
        None => {
            println!("\nWe don't know a goat by that name.");
            println!("{trimmed_name} is a great name for a goat!");
            record_goat(trimmed_name, goat_map, lowercased_goat_name);
        }
    }
}

fn get_goat_name() -> String {
    get_string_from_stdin("Please name a goat.".to_string())
}

fn get_string_from_stdin(message: String) -> String {
    let mut trimmed_name = String::new();

    // Prevents "enter" from the input from making the output appearing on more than one line
    loop {
        println!("\n{message}");
        let mut input_text_butter = String::new();
        let read_result = io::stdin().read_line(&mut input_text_butter);

        match read_result {
            Ok(_) => {
                trimmed_name = input_text_butter.trim().chars().collect();
            }
            Err(_) => (),
        }
        if trimmed_name == "" {
            println!("I didn't get that.");
        } else {
            break;
        }
    }
    trimmed_name
}

fn record_goat(
    trimmed_name: String,
    goat_map: &mut HashMap<String, Goat>,
    lowercased_goat_name: String,
) {
    let new_goat = Goat {
        name: trimmed_name,
        power_level: get_power_level(),
        is_grumpy: get_is_grumpy(),
    };
    println!("\nThis is a good goat!  I will add it to the database.\n\nNew goat entry: ");
    new_goat.log();
    goat_map.insert(lowercased_goat_name, new_goat);
}

fn get_is_grumpy() -> bool {
    let is_grumpy: bool;
    let mut input_text_butter = String::new();
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
    is_grumpy
}

fn get_power_level() -> u32 {
    let power_level: u32;
    let mut input_text_butter = String::new();
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
    power_level
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_power_level_label() {
        let mut test_goat = Goat {
            name: "Gloria".to_string(),
            power_level: 3,
            is_grumpy: false,
        };
        assert_eq!(
            test_goat.get_power_level_label(),
            "not very powerful",
            "3 case"
        );
        test_goat.power_level = 67;
        assert_eq!(
            test_goat.get_power_level_label(),
            "pretty powerful",
            "67 case"
        );
        test_goat.power_level = 144;
        assert_eq!(
            test_goat.get_power_level_label(),
            "SUPER powerful",
            "144 case"
        );
        test_goat.power_level = 287;
        assert_eq!(
            test_goat.get_power_level_label(),
            "super duper powerful",
            "287 case"
        );
        test_goat.power_level = 350;
        assert_eq!(
            test_goat.get_power_level_label(),
            "EXTREEEMELY powerful",
            "350 case"
        );
    }
}

use std::io;

struct Goat {
    pub name: String,
    pub power_level: u16,
    pub is_grumpy: bool,
}

impl Goat {
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

    let goats = [
        Goat {
            name: "Gruf".to_string(),
            power_level: 999,
            is_grumpy: true,
        },
        Goat {
            name: "Fawn".to_string(),
            power_level: 2,
            is_grumpy: false,
        },
        Goat {
            name: "Billy".to_string(),
            power_level: 32,
            is_grumpy: true,
        },
        Goat {
            name: "George".to_string(),
            power_level: 117,
            is_grumpy: false,
        },
    ];
    // TODO:  Convert this array of goats to a Hash map of Strings and goats so the input reader can look a goat up by name.

    for goat in goats {
        if !goat.is_grumpy {
            goat.log();
        }
    }

    // Takes a user-supplied name, then tells them it's great.  Is it?  Who knows...
    println!("Please name a goat.");

    let mut supplied_name = String::new();

    io::stdin()
        .read_line(&mut supplied_name)
        .expect("I didn't get that.");

    // Prevents "enter" for goat name from making the output appearing on more than one line
    let trimmed_name = supplied_name.trim();
    println!("{trimmed_name} is a great name for a goat!");
}

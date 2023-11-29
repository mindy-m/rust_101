struct Goat {
    pub name: String,
    pub power_level: u16,
    pub is_grumpy: bool,
}

/// Prints a number of goats.
fn main() {
    let mut goat_count: u8 = 5;
    goat_count += 7;

    // Math is done...
    println!("We have {goat_count} goats!");

    let gruf = Goat {
        name: "Gruf".to_string(),
        power_level: 72,
        is_grumpy: true,
    };

    let fawn = Goat {
        name: "Fawn".to_string(),
        power_level: 2,
        is_grumpy: false,
    };

    log_goat(&gruf);
    log_goat(&fawn);
}

fn log_goat(goat: &Goat) {
    let grumpy_label = if goat.is_grumpy {
        "is grumpy"
    } else {
        "is super happy"
    };

    println!(
        "{0} has a power level of {1}, and {grumpy_label}.",
        goat.name, goat.power_level
    )
}

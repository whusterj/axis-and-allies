use rand::Rng;
use std::cmp::min;
use std::env;
use std::time::Instant;

enum UnitType {
    Infantry,
    Tank,
    Fighter,
    Bomber,
    Battleship,
}

struct Unit {
    unit_type: UnitType,
    ipc: u8,
    attack: u8,
    defend: u8,
}

const INFANTRY: Unit = Unit {
    unit_type: UnitType::Infantry,
    ipc: 3,
    attack: 1,
    defend: 2,
};

const TANK: Unit = Unit {
    unit_type: UnitType::Tank,
    ipc: 6,
    attack: 3,
    defend: 3,
};

const FIGHTER: Unit = Unit {
    unit_type: UnitType::Fighter,
    ipc: 10,
    attack: 3,
    defend: 4,
};

const BOMBER: Unit = Unit {
    unit_type: UnitType::Bomber,
    ipc: 12,
    attack: 4,
    defend: 1,
};

const BATTLESHIP: Unit = Unit {
    unit_type: UnitType::Battleship,
    ipc: 24,
    attack: 4,
    defend: 4,
};

fn rand_between(min: u8, max: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure there are at least two arguments provided
    if args.len() < 4 {
        eprintln!("Usage: {} <attacker_units> <defender_units>", args[0]);
        std::process::exit(1);
    }

    let attacker_units = create_units_vector(&args[1]);
    let defender_units = create_units_vector(&args[2]);
    let sim_count = args[3].parse::<u32>().unwrap();

    print_units(&attacker_units, "Attacker");
    print_units(&defender_units, "Defender");

    let start_time = Instant::now();

    let mut a_wins = 0;
    let mut d_wins = 0;
    let mut draws = 0;
    for _ in 0..sim_count {
        let mut done = false;
        let mut a_casualties = 0;
        let mut d_casualties = 0;
        while !done {
            // Roll for each attacking unit, and
            // tally new defender casualties -- these
            // are not applied until the end of the round.
            let mut new_d_casualties = 0;
            for i in a_casualties..attacker_units.len() {
                let roll = rand_between(1, 6);
                let unit = attacker_units[i];
                if roll <= unit.attack {
                    new_d_casualties += 1;
                }
            }

            // Roll for each defending unit and
            // immediately take attacker casualties
            let mut new_a_casualties = 0;
            for i in d_casualties..defender_units.len() {
                let roll = rand_between(1, 6);
                let unit = defender_units[i];
                if roll <= unit.defend {
                    new_a_casualties += 1;
                }
            }

            // Don't take more casualties than number of units
            a_casualties = min(a_casualties + new_a_casualties, attacker_units.len());
            d_casualties = min(d_casualties + new_d_casualties, defender_units.len());

            if (a_casualties == attacker_units.len()) || (d_casualties == defender_units.len()) {
                done = true;
            }
        }

        if a_casualties == attacker_units.len() && d_casualties == defender_units.len() {
            draws += 1;
        } else if a_casualties == attacker_units.len() {
            d_wins += 1;
        } else if d_casualties == defender_units.len() {
            a_wins += 1;
        }
    }

    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("--- Results ---");
    println!("Execution time: {:?}", duration);
    println!("Attacker Wins: {}", a_wins);
    println!("Defender Wins: {}", d_wins);
    println!("Draws: {}", draws);
    println!("Win Ratio: {:.2}", a_wins as f32 / sim_count as f32);
}

fn create_units_vector(input: &str) -> Vec<&Unit> {
    let mut units: Vec<&Unit> = Vec::new();
    for c in input.chars() {
        match c {
            'I' => units.push(&INFANTRY),
            'F' => units.push(&FIGHTER),
            'T' => units.push(&TANK),
            'b' => units.push(&BOMBER),
            'B' => units.push(&BATTLESHIP),
            _ => println!("Unknown character: {}", c),
        }
    }
    units
}

fn print_units(units: &Vec<&Unit>, side: &str) {
    println!("{} Units:", side);
    for unit in units {
        match unit.unit_type {
            UnitType::Infantry => println!("  Infantry"),
            UnitType::Fighter => println!("  Fighter"),
            UnitType::Tank => println!("  Tank"),
            UnitType::Bomber => println!("  Bomber"),
            UnitType::Battleship => println!("  Battleship"),
        }
    }
}

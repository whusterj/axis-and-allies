use rand::Rng;
use std::cmp::min;
use std::time::{Duration, Instant};

// TODO: Delete the following or replace with my own tests
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
// END TODO

enum UnitType {
    Infantry,
    Tank,
    Fighter,
    Bomber,
    Battleship,
}

pub struct Unit {
    unit_type: UnitType,
    ipc: u8,
    attack: u8,
    defend: u8,
}

pub const INFANTRY: Unit = Unit {
    unit_type: UnitType::Infantry,
    ipc: 3,
    attack: 1,
    defend: 2,
};

pub const TANK: Unit = Unit {
    unit_type: UnitType::Tank,
    ipc: 6,
    attack: 3,
    defend: 3,
};

pub const FIGHTER: Unit = Unit {
    unit_type: UnitType::Fighter,
    ipc: 10,
    attack: 3,
    defend: 4,
};

pub const BOMBER: Unit = Unit {
    unit_type: UnitType::Bomber,
    ipc: 12,
    attack: 4,
    defend: 1,
};

pub const BATTLESHIP: Unit = Unit {
    unit_type: UnitType::Battleship,
    ipc: 24,
    attack: 4,
    defend: 4,
};

pub struct SimulationResult {
    pub attacker_wins: u32,
    pub defender_wins: u32,
    pub draws: u32,
    pub duration: Duration,
}

fn rand_between(min: u8, max: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn simulate(
    mut attacker_units: Vec<&Unit>,
    mut defender_units: Vec<&Unit>,
    sim_count: u32,
) -> SimulationResult {
    // Sort units by IPC value in ascending order. This ensures that
    // the least valuable units are the first to be taken as casualties.
    attacker_units.sort_by_key(|unit| unit.ipc);
    defender_units.sort_by_key(|unit| unit.ipc);

    let start_time = Instant::now();

    let mut a_wins = 0;
    let mut d_wins = 0;
    let mut draws = 0;
    for _ in 0..sim_count {
        let mut done = false;
        let mut a_casualties = 0;
        let mut d_casualties = 0;
        while !done {
            // Roll for each attacking unit that is not a casualty.
            let mut new_d_casualties = 0;
            for i in a_casualties..attacker_units.len() {
                let roll = rand_between(1, 6);
                let unit = attacker_units[i];
                if roll <= unit.attack {
                    new_d_casualties += 1;
                }
            }

            // Roll for each defending unit that is not a casualty.
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

    return SimulationResult {
        attacker_wins: a_wins,
        defender_wins: d_wins,
        draws: draws,
        duration: duration,
    };
}

pub fn create_units_vector(input: &str) -> Vec<&Unit> {
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

pub fn print_units(units: &Vec<&Unit>, side: &str) {
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

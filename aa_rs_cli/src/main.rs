use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure there are at least two arguments provided
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <attacker_units> <defender_units> <sim_count>",
            args[0]
        );
        std::process::exit(1);
    }

    let attacker_units = aa_rs::create_units_vector(&args[1]);
    let defender_units = aa_rs::create_units_vector(&args[2]);
    let sim_count = args[3].parse::<u32>().unwrap();

    aa_rs::print_units(&attacker_units, "Attacker Units");
    aa_rs::print_units(&defender_units, "Defender Units");

    let result: aa_rs::SimulationResult =
        aa_rs::simulate(attacker_units, defender_units, sim_count);

    println!("--- Results ---");
    println!("Execution time: {:?}", result.duration);
    println!("Attacker Wins: {}", result.attacker_wins);
    println!("Defender Wins: {}", result.defender_wins);
    println!("Draws: {}", result.draws);
    println!(
        "Win Ratio: {:.2}",
        result.attacker_wins as f32 / sim_count as f32
    );
}

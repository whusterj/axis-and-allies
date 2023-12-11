// Set up unit types
struct Unit {
    unit_type: String,
    ipc: u8,
    attack: u8,
    defend: u8,
}

fn build_unit(unit_type: String, ipc: u8, attack: u8, defend: u8) -> Unit {
    return Unit {
        unit_type,
        ipc,
        attack,
        defend,
    };
}

fn main() {
    let infantry = build_unit(String::from("Infantry"), 3, 1, 2);
    let tank = build_unit(String::from("Tank"), 6, 3, 3);
    let fighter = build_unit(String::from("Fighter"), 10, 3, 4);
    let bomber = build_unit(String::from("Bomber"), 10, 4, 1);
    let battleship = build_unit(String::from("Battleship"), 10, 4, 4);

    println!("{}", infantry.unit_type);
}

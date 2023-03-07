use std::io;
use rand::Rng;

#[derive(Debug)]
#[allow(dead_code)]
enum Race {
    Dwarf,
    Elf,
    Halfling,
    Human,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling
}

#[derive(Debug)]
#[allow(dead_code)]
enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcer,
    Warlock,
    Wizard
}

#[allow(non_snake_case)]
struct CharacterSheet {
    Name: String,
    Age: u16,
    Race: Race,
    Class: Class,
    AbilityScores: [i8; 6],
    AbilityModifiers: [i8; 6],
}

fn main_menu(cs: &CharacterSheet) -> bool {
    println!("| n - Change Name | r - Change Race | c - Change Class | s - Save Character | q - Quit |");
    let select = user_input();
    // I do not know what I am doing
    let mut is_quit: bool = false;
    match select {
        'n' => println!("Change Name"),
        'r' => println!("Change Race"),
        'c' => println!("Change Class"),
        's' => println!("Save Charactersheet"),
        'd' =>  {
                    clear_screen();
                    display_charactersheet(cs);
                },
        'q' =>  {
                    is_quit = true;
                    println!("Quitting Program");
                },
        _ => println!("Invalid Selection")
    }
    is_quit
}

fn user_input() -> char {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // We only want the first letter in the string
    
    let c = input.chars().next().unwrap();
    c
}

fn new_character(name: String, race: Race, class: Class) -> CharacterSheet {
    let scores: [i8; 6] = _generate_ability_scores();
    let mods: [i8; 6] = _generate_ability_modifiers(scores);
    CharacterSheet { 
        Name: (name), 
        Age: (2023 - 1997), 
        Race: (race), 
        Class: (class), 
        AbilityScores: (scores), 
        AbilityModifiers: (mods) }
}

fn display_charactersheet(cs: &CharacterSheet) {
    // Cringe::Start
    println!();
    println!("|        Name: {:?}", cs.Name);
    println!("|         Age: {:?}", cs.Age);
    println!("|        Race: \"{:?}\"", cs.Race);
    println!("|       Class: \"{:?}\"", cs.Class);
    println!("|    Strength: {}({})", cs.AbilityScores[0], cs.AbilityModifiers[0]);
    println!("|   Dexterity: {}({})", cs.AbilityScores[1], cs.AbilityModifiers[1]);
    println!("|Constitution: {}({})", cs.AbilityScores[2], cs.AbilityModifiers[2]);
    println!("|Intelligence: {}({})", cs.AbilityScores[3], cs.AbilityModifiers[3]);
    println!("|      Widsom: {}({})", cs.AbilityScores[4], cs.AbilityModifiers[4]);
    println!("|    Charisma: {}({})", cs.AbilityScores[5], cs.AbilityModifiers[5]);
    // Cringe::End
}

fn _roll_1d6() -> i8 {
    rand::thread_rng().gen_range(1..=6)
}

fn _generate_ability_scores() -> [i8; 6] {
    let mut scores: [i8; 6] = [0, 0, 0, 0, 0, 0];

    for i in 0..scores.len() {
        let mut arr: [i8; 4] = [_roll_1d6(), _roll_1d6(), _roll_1d6(), _roll_1d6()];
        bubble_sort(&mut arr);
        scores[i] = arr[1] + arr[2] + arr[3];
    }   
    
    scores
}

fn _generate_ability_modifiers(scores: [i8; 6]) -> [i8; 6] {
    [scores[0] / 2 - 5,scores[1] / 2 - 5,scores[2] / 2 - 5,scores[3] / 2 - 5,scores[4] / 2 - 5,scores[5] / 2 - 5]
}

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let name:String = String::from("Kaleb");
    let race: Race = Race::Human;
    let class: Class = Class::Fighter;
    let cs: CharacterSheet = new_character(name, race, class);
    display_charactersheet(&cs);
    loop {
        // print!("\x1B[2J\x1B[1;1H");
        let is_quit = main_menu(&cs);
        if is_quit {
            break;
        }
    }
}
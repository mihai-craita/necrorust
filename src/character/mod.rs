use std::fmt::Display;
use rand::Rng;
use core::fmt;
use colored::Colorize;
use colored::ColoredString;

#[derive(Clone)]
pub struct Character {
    hp: i32,
    max_hp: i32,
    min_attack: i32,
    max_attack: i32,
    name: ColoredString,
    pub experience: usize,
}

impl Character {
    pub fn display_character(&self) -> String {
        format!("{} | HP: {}/{} | ATK: {}-{}| XP: {}",
                self.name(),
                self.hp.to_string().red(),
                self.max_hp,
                self.min_attack,
                self.max_attack, 
                self.experience.to_string().blue()
                )
    }
    pub fn add_min_attack(&mut self, a: i32) {
        self.min_attack += a;
    }
    pub fn add_max_attack(&mut self, a: i32) {
        self.max_attack += a;
    }
    pub fn add_max_health(&mut self, a: i32) {
        self.max_hp += a;
    }
    pub fn add_experience(&mut self, xp: usize) {
        self.experience += xp;
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
    fn hit(&mut self, attack: i32) {
        self.hp -= attack;
    }
    pub fn full_heal(&mut self) {
        self.hp = self.max_hp;
    }
    pub fn heal(&mut self, h: i32) {
        self.hp += h;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
    pub fn attack(&self, oponnent: &mut Character) {
        let attack = rand::thread_rng().gen_range(self.min_attack..=self.max_attack);
        oponnent.hit(attack);
        println!("⚔️  -> {} deals {} damage to {} (hp {})", self.name, attack.to_string().red(), oponnent.name(), oponnent.hp());
    }
    pub fn name(&self) -> &ColoredString {
        &self.name
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} hp: {} / {} ", self.name, self.hp, self.max_hp)
    }
}

// "Dark Drakes": Small, winged dragon-like creatures that lurk in the shadows and spew poisonous fumes, disorienting their victims.

// Deathsworn Rats: Giant, mutated rats carrying deadly diseases and swarming in large numbers, overwhelming their prey.

//     Shadow Serpents: Enormous, slithering snakes capable of squeezing the life out of their victims while hiding in the darkness.

//     Gravewalkers: Decayed corpses reanimated by dark magic, relentlessly stalking their prey with an insatiable hunger for the living.

//     Bloodsucking Bats: Vampiric bats that drain the life force of their victims, leaving them weakened and vulnerable.

//     Crypt Crawlers: Grotesque, insectoid creatures with razor-sharp mandibles that can quickly burrow through the dungeon's earthen floors and walls.

//     Soul Trappers: Ghostly beings that ensnare the souls of the fallen, using them as a source of power to fuel their own dark energies.

pub fn new_monster(name: &str) -> Character {
    match name {
        "Gloomhound"           => new_character(20, 25, 2,  4,  String::from("Gloomhound").red(), 7),
        "Dark Drakes"          => new_character(30, 30, 3, 5, String::from("Dark Drakes").red(), 10),
        "Deathsworn Rats"      => new_character(20, 20, 1, 4, String::from("Deathsworn Rats").red(), 6),
        "Shadow Serpents"      => new_character(40, 40, 1, 3, String::from("Shadow Serpents").red(), 10),
        "Gravewalker"          => new_character(40, 40, 1, 4, String::from("Gravewalker").red(), 12),
        "Soul Trappers"        => new_character(20, 20, 3, 8, String::from("Soul Trappers").red(), 8),
        "Gorefang the Ravager" => new_character(100, 100, 5, 20, String::from("Gorefang the Ravager").red(), 33),
        _ => new_character(30, 30, 2, 4, String::from("unknown ghost").red(), 0)
    }
}

pub fn new_hero() -> Character {
    Character {
        hp: 100,
        max_hp: 100,
        min_attack: 2,
        max_attack: 5,
        name: String::from("Lyra").bold().bright_blue(),
        experience: 0
    }
}

pub fn new_character(
    hp: i32,
    max_hp: i32,
    min_attack: i32,
    max_attack: i32,
    name: ColoredString,
    xp: usize
    ) -> Character {
    Character { hp, max_hp, min_attack, max_attack, name, experience: xp}
}

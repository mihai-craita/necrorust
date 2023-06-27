use crate::character::Character;
use crate::character::new_monster;
use rand::Rng;

type TurnNumber = usize;

pub struct Dungeon {
    pub name: String,
    pub turn: TurnNumber,
    pub max_turn: TurnNumber,
    pub no_of_monsters: usize,
    pub final_boss: Character,
}

impl Dungeon {
    pub fn next_monster(&self) -> Character {
        if self.is_last_turn() {
            new_monster("Gorefang the Ravager")
        } else {
            let monster_names = vec!["Dark Drakes", "Gravewalker", "Gloomhound", "Deathsworn Rats", "Shadow Serpents", "Soul Trappers"];
            let a = rand::thread_rng().gen_range(0..monster_names.len());
            let monster_name = monster_names.get(a).unwrap();
            new_monster(monster_name)
        }
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    fn is_last_turn(&self) -> bool {
        self.turn == self.max_turn
    }

    pub fn ended(&self) -> bool {
        self.turn > self.max_turn
    }

    pub fn show_info(&self) {
            println!("
Entering the {}.
--------------------------------------------
This Dungeon has 6 types of monsters you will fight for {} turns.
Final Boss: {}, a fearsome beast with razor-sharp claws.
{}.", self.name, self.max_turn, self.final_boss.name(), self.final_boss.display_character());
    }
}

pub fn new_dungeon() -> Dungeon {
    Dungeon {
        name: "Shadowcrypt of Despair Dungeon".to_string(),
        turn: 0,
        max_turn: 20,
        no_of_monsters: 20,
        final_boss: new_monster("Gorefang the Ravager")
    }
}

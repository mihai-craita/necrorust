use crate::character::Character;
use crate::character::{new_monster, HasName};
use rand::Rng;

pub struct Dungeon {
    pub name: String,
    pub turn: usize,
    pub no_of_monsters: usize,
    pub final_boss: Character,
}

impl Dungeon {
    pub fn next_monster(&self) -> Character {

        if self.turn > self.no_of_monsters {
            new_monster("Gorefang the Ravager")
        } else {
            let monster_names = vec!["Dark Drakes", "Gravewalker", "Gloomhound", "Deathsworn Rats", "Shadow Serpents", "Soul Trappers"];
            let a = rand::thread_rng().gen_range(0..monster_names.len());
            let monster_name = monster_names.get(a).unwrap();
            new_monster(monster_name)
        }

    }

    pub fn next_turn(&mut self) {
        self.turn = self.turn + 1;
    }

    pub fn show_info(&self) {
            println!("
Entering the {}.
--------------------------------------------
This Dungeon has 6 types of monsters you will fight {} minions and one final boss.
Final Boss: {}, a fearsome beast with razor-sharp claws.
{}.", self.name, self.no_of_monsters, self.final_boss.name(), self.final_boss.display_character());
    }
}

pub fn new_dungeon() -> Dungeon {
    Dungeon {
        name: "Shadowcrypt of Despair Dungeon".to_string(),
        turn: 0,
        no_of_monsters: 20,
        final_boss: new_monster("Gorefang the Ravager")
    }
}

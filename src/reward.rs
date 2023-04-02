use crate::character::{Character, HasLife};

pub trait Reward {
    fn display(&self) -> String;
    fn reward_hero(&self, hero: &mut Character);

}

pub struct FullHeal { }

impl Reward for FullHeal {
    fn display(&self) -> String {
        "full heal".to_string()
    }
    fn reward_hero(&self, hero: &mut Character) {
        hero.full_heal();
    }
}

pub struct HealGainMaxHpMaxAttack {
    pub heal: i32,
    pub max_hp: i32,
    pub max_attack: i32
}

impl Reward for HealGainMaxHpMaxAttack {
    fn display(&self) -> String {
        format!("heal {}, gain +{} max HP, +{} max attack", self.heal, self.max_hp, self.max_attack)
    }
    fn reward_hero(&self, hero: &mut Character) {
        hero.heal(self.heal);
        hero.add_max_health(self.max_hp);
        hero.add_max_attack(self.max_attack); 
    }
}

pub struct HealGainMinAttackMaxAttack {
    pub heal: i32,
    pub min_attack: i32,
    pub max_attack: i32
}

impl Reward for HealGainMinAttackMaxAttack {
    fn display(&self) -> String {
        format!("heal {}, +{} min attack, +{} max attack", self.heal, self.min_attack, self.max_attack)
    }
    fn reward_hero(&self, hero: &mut Character) {
        hero.heal(self.heal);
        hero.add_min_attack(self.min_attack);
        hero.add_max_attack(self.max_attack); 
    }
}

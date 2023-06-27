use std::io::Read;
use std::io;
use crate::character::*;
use crate::dungeon::new_dungeon;
use crate::reward::*;
use rand::seq::SliceRandom;

pub mod character;
pub mod game;
pub mod reward;
pub mod dungeon;
pub mod fight;

fn main() {
    let mut dungeon = new_dungeon();

    let mut hero = new_hero();

    while hero.is_alive() {
        std::thread::sleep(std::time::Duration::from_millis(500));

        if dungeon.turn == 0 {
            dungeon.show_info();

            press_enter_to_continue();
        }

        // get a new monster
        let mut monster = dungeon.next_monster();

        println!("Your character info: {} ", hero.display_character());
        println!("Next monster: {}!", monster.display_character());
        press_enter_to_continue();
        if dungeon.turn > 0 {
            choose_reward(&mut hero);
        }

        fight::fight(&mut monster, &mut hero);
        if hero.is_dead() {
            println!("Game over you lost!");
        } else if monster.is_dead() {
            hero.add_experience(monster.experience);
            dungeon.next_turn();
            println!("Dungeon Turn: {} - You defeated monster {}!\n", dungeon.turn, monster.name());
            if dungeon.ended() {
                println!("Dungeon ended!");
                println!("You won!");
            }
        }
    }
}

fn choose_reward(hero: &mut Character) {
    let possible_rewards: Vec<Box<dyn Reward>> = vec![
        Box::new(FullHeal {}),
        Box::new(HealGainMaxHpMaxAttack{ heal: 5, max_hp: 5, max_attack: 1}),
        Box::new(HealGainMinAttackMaxAttack{ heal: 8, min_attack: 1, max_attack: 1}),
        Box::new(HealGainMaxAttack{ heal: 8, max_attack: 2}),
        Box::new(HealGainMinAttack{ heal: 8, min_attack: 2}),
    ];

    let mut rng = &mut rand::thread_rng();
    let rewards: Vec<&Box<dyn Reward>> = possible_rewards.choose_multiple(&mut rng, 3).collect();

    loop {
        println!("
-----------------
Choose one reward:
-----------------");
        let mut i = 0;
        for reward in rewards.iter() {
            i += 1;
            println!("{} {}", i, reward.display());
        };

        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(_) => {
                // input = input.trim().to_;
                let input = input.trim_matches('\n');
                let option_choose = input.parse::<usize>();
                match option_choose {
                    Ok(val) => {
                        if val > 3 {
                            println!("You choose a wrong reward choose between 1 or 2 or 3");
                        } else {
                            let val: usize = val - 1;
                            let r = rewards.get(val);
                            match r {
                                Some(reward) => reward.reward_hero(hero),
                                None => println!("Something wrong happened you reward was not found")
                            };
                            break;
                        }
                    },
                    Err(_) => {
                        println!("You choose a wrong reward choose between 1 or 2 or 3");
                    }
                }
            },
            Err(_) => {
                println!("Err: We could not read from input");
            }
        }
    }
}

fn press_enter_to_continue() {
    println!("\nPress ENTER to continue...");
    let buffer = &mut [0u8];
    io::stdin().read_exact(buffer).unwrap();
}

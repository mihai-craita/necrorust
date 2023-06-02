use std::fmt::Display;

use crate::character::{HasLife, Attacker, HasName};

pub fn fight<T>(first_contender: &mut T, second_contender: &mut T)
where
T: HasLife + Attacker + HasName + Display
{
    // fight loop
    let mut turn = 0;
    loop {
        turn += 1;

        first_contender.attack(second_contender);
        if second_contender.is_dead() {
            break;
        }
        second_contender.attack(first_contender);
        println!("Turn {}: {} -> {} \n", turn, second_contender, first_contender);
        if first_contender.is_dead() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

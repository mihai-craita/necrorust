use crate::character::Character;

pub fn fight(first_contender: &mut Character, second_contender: &mut Character)
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

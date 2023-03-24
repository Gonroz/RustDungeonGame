mod character;
mod spell;

fn main() {
    println!("Hello, world!");
    let mut player = character::Character::new(); 
    player.learn_new_spell(spell::Spell::new_random_spell(), 0);
    println!("{:?}", player);
    let mut enemy = character::Character::new_enemy();
    println!("{:?}", enemy);
    while enemy.health > 0 {
        player.attack(&mut enemy);
    }
    println!("You have killed {}!", enemy.name);
}

use crate::spell::Spell;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub health: i32,
    pub level: u32,
    pub experience: u32,

    pub omega: u32,
    pub zeta: u32,
    pub gamma: u32,
    pub constitution: u32,

    pub spell_list: [Option<Spell>; 6],
}

impl Character {
    pub fn new() -> Self {
        Self {
            name: "bob".to_string(),
            health: 21,
            level: 1,
            experience: 0,
            gamma: 1,
            omega: 6,
            zeta: 4,
            constitution: 5,
            spell_list: Default::default(), // defaults move list to all none values
        }
    }

    pub fn new_enemy() -> Self {
        Self {
            name: "Cheese Thrower".to_string(),
            health: 21,
            level: 1,
            experience: 0,
            gamma: 1,
            omega: 6,
            zeta: 4,
            constitution: 5,
            spell_list: Default::default(), // defaults move list to all none values
        }
    }

    pub fn learn_new_spell(&mut self, spell: Spell, spell_slot: usize) {
        self.spell_list[spell_slot] = Some(spell);
    }

    pub fn attack(&self, target: &mut Character) {
        println!("Attacking {} for {} damage!", target.name, 3);
        target.health = target.health - 3;
    }
}

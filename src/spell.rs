#[derive(Debug)]
pub struct Spell {
    pub damage: i32,
    pub accuracy: u8,
    //pub pp or uses?
    //pub status_effect: Option<StatusEffect>,
    pub status: Option<Status>,
    pub gamma_scaling: f32,
    pub omega_scaling: f32,
    pub zeta_scaling: f32,
}

impl Spell {
    pub fn new_random_spell() -> Self {
        Spell {
            damage: 10,
            accuracy: 90,
            gamma_scaling: 1.05,
            omega_scaling: 1.10,
            zeta_scaling: 1.3,
            //status_effect: None,
            status: Self::random_status_effect(),
        }
    }

    fn random_status_effect() -> Option<Status> {
        return Some(Status(StatusEffect::Burning, 5.0));
    }
}

#[derive(Debug)]
pub struct Status(StatusEffect, f32);

#[derive(Debug)]
pub enum StatusEffect {
    Bleeding,
    Burning,
    Freezing,
    Withered,
    Blinded,
    Poisoned,
}

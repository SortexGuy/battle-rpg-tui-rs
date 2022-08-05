use crate::{ui_rendering::ActionOptions, State};
use rand::random;

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub attack: u16,
    pub defense: u16,
    pub hope: u16,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
    pub health: u16,
    pub max_health: u16,
    pub mana: u16,
    pub max_mana: u16,
    pub time: f32,
    pub time_mod: f32,
    pub act_available: Vec<ActionOptions>,
}
impl Default for Character {
    fn default() -> Self {
        let mut chara = Character {
            name: "Character".to_string(),
            stats: Stats::default(),
            health: 0,
            max_health: 100,
            mana: 0,
            max_mana: 100,
            time: 0.0,
            time_mod: 1.0 + (random::<f32>() % 2.0),
            act_available: vec![
                ActionOptions::Attack,
                ActionOptions::Defend,
                ActionOptions::Ability,
            ],
        };
        chara.health = chara.max_health;
        chara.mana = chara.max_mana;
        chara
    }
}
impl Character {
    pub fn update(&mut self, delta: f32) {
        let mut time = self.time + delta * self.time_mod;
        if time > 60.0 {
            time = 60.0;
        }
        self.time = time;
    }
    // To add an action to the player
    pub fn add_action(&mut self, act: &ActionOptions) {
        // If it already is in the actions vector, do nothing
        if self.act_available.contains(act) { return; }
        // Else push the action and sort the vector
        self.act_available.push(*act);
        self.act_available.sort_unstable_by(|a, b|
            (*a as usize).cmp(&(*b as usize))
        );
    }
}

pub fn update_chars_time(state: &mut State, delta: f32) {
    for party in vec![&mut state.player_party, &mut state.enemy_party].iter_mut() {
        for chara in party.iter_mut() {
            chara.update(delta);
        }
    }
}

use crate::BattleState;
use rand::random;
use std::{
    fmt::{self, Display},
    vec,
};

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub attack: u16,
    pub defense: u16,
    pub hope: u16,
}
// dmg = tu_attack * (rand * tu_hope) - enemy_defense * (rand * enemy_hope)

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Commands {
    Attack,
    Defend,
    Magic,
    Ability,
    Manif,
    Max,
}
impl Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Actions {
    name: String,
    damage: u16,
    duration: f32,
    time_cost: f32,
    mana_cost: u16,
}
impl Default for Actions {
    fn default() -> Self {
        Actions {
            name: "Action".to_string(),
            damage: 0,
            duration: 0.,
            time_cost: 0.,
            mana_cost: 0,
        }
    }
}
impl Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
    // last_health: u16,
    pub health: u16,
    pub max_health: u16,
    pub mana: u16,
    pub max_mana: u16,
    pub time: f32,
    pub time_mod: f32,
    pub cmd_available: Vec<Commands>,
    pub act_available: [Vec<Actions>; Commands::Max as usize],
}
impl Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Default for Character {
    fn default() -> Self {
        let mut chara = Character {
            name: "Character".to_string(),
            stats: Stats::default(),
            // last_health: 100,
            health: 100,
            max_health: 100,
            mana: 100,
            max_mana: 100,
            time: 0.0,
            time_mod: 1.0 + (random::<f32>() % 2.0),
            cmd_available: vec![Commands::Attack, Commands::Defend, Commands::Ability],
            act_available: [
                // Attack
                vec![Actions::default()],
                // Defend
                vec![Actions::default()],
                // Magic
                vec![Actions::default(), Actions::default()],
                // Ability
                vec![Actions::default(), Actions::default()],
                // Manifestations
                vec![Actions::default()],
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
    pub fn add_action(&mut self, act: &Commands) {
        // If it already is in the actions vector, do nothing
        if self.cmd_available.contains(act) {
            return;
        }
        // Else push the action and sort the vector
        self.cmd_available.push(*act);
        self.cmd_available.sort_unstable_by_key(|a| *a as usize);
    }
}

pub fn update_chars_time(state: &mut BattleState, delta: f32) {
    for party in vec![&mut state.player_party, &mut state.enemy_party].iter_mut() {
        for chara in party.iter_mut() {
            chara.update(delta);
        }
    }
}

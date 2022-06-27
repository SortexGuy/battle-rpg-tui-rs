use crate::State;

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub attack: u16,
    pub defense: u16,
    pub hope: u16,
}

#[derive(Debug, Default, Clone)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
    pub health: u16,
    pub max_health: u16,
    pub mana: u16,
    pub max_mana: u16,
    pub time: f32,
    pub time_mod: f32,
}
impl Character {
    pub fn update(&mut self, delta: f32) {
        let mut time = self.time + delta * self.time_mod;
        if time > 60.0 {
            time = 60.0;
        }
        self.time = time;
    }
}

pub fn update_chars_time(state: &mut State, delta: f32) {
    for chara in state.player_party.iter_mut() {
        chara.update(delta);
    }
}


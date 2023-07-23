use memcs_core::sdk::csgo::SignOnState;
use memflow::types::Address;

#[derive(Clone, Copy)]
pub struct Config {
    pub glow: bool,
    pub bhop: bool,
    pub radar: bool,
    pub chams: bool,
}

#[derive(Clone)]
pub struct PlayerInfo {
    name: Option<String>,
    is_enemy: bool,
    is_local: bool,
    is_alive: bool,
}

impl PlayerInfo {
    pub fn new(name: Option<String>, is_enemy: bool, is_local: bool, is_alive: bool) -> Self {
        Self {
            name, is_enemy, is_local, is_alive
        }
    }

    pub fn default() -> Self {
        Self {
            name: None,
            is_enemy: false,
            is_local: false,
            is_alive: false,
        }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn is_enemy(&self) -> bool {
        self.is_enemy
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}

#[derive(Clone)]
pub struct SharedData {
    pub player_list: Vec<PlayerInfo>,
    pub elapsed_time: Option<std::time::Duration>,
    pub engine_base: Option<Address>,
    pub client_base: Option<Address>,
    pub config: Config,
    pub chams_once: bool,
    pub should_exit: bool,
    pub game_state: Option<SignOnState>,
}

impl SharedData {
    pub fn default() -> Self {
        Self {
            player_list: vec![],
            elapsed_time: None,
            engine_base: None,
            client_base: None,
            config: Config {
                glow: false,
                bhop: false,
                radar: false,
                chams: false,
            },
            chams_once: false,
            should_exit: false,
            game_state: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    brightness: f32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, brightness: f32) -> Color {
        Color {
            r, g, b, brightness
        }
    }

    pub fn red(&self) -> u8 {
        self.r
    }

    pub fn green(&self) -> u8 {
        self.g
    }

    pub fn blue(&self) -> u8 {
        self.b
    }

    pub fn brightness(&self) -> f32 {
        self.brightness
    }
}
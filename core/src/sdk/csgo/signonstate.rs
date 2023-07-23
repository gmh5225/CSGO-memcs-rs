#[derive(FromPrimitive, ToPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignOnState {
    None = 0,
    Challenge,
    Connected,
    New,
    Prespawn,
    Spawn,
    Full,
    ChangeLevel
}

impl SignOnState {
    pub fn as_str(&self) -> &str {
        match self {
            SignOnState::None => "disconnected",
            SignOnState::Challenge => "challenge",
            SignOnState::Connected => "connected",
            SignOnState::New => "new",
            SignOnState::Prespawn => "prespawn",
            SignOnState::Spawn => "spawn",
            SignOnState::Full => "ingame",
            SignOnState::ChangeLevel => "change level",
        }
    }
}

impl core::fmt::Display for SignOnState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
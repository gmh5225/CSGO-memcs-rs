use memflow::{types::Address, prelude::MemoryView};

use crate::{CheatCtx, sdk::error::Error};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct PlayerInfo {
    pub addr: Address
}

impl PlayerInfo {
    pub fn get_name(&self, ctx: &mut CheatCtx) -> Result<String, Error> {
        Ok(ctx.process.read_char_string_n(self.addr + 0x10, 128)?)
    }
}

unsafe impl dataview::Pod for PlayerInfo {}

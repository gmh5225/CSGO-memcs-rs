use memflow::{prelude::MemoryView, types::Address};

use crate::{
    cheat_ctx::CheatCtx,
    sdk::{
        error::Error,
        structs::{PlayerInfo, Vec3},
    },
};

pub struct ClientState {
    pub ptr: Address,
}

impl ClientState {
    pub fn get(ctx: &mut CheatCtx) -> Result<ClientState, Error> {
        let offset = ctx.offsets.get_sig("dwClientState")?;
        let ptr = ctx.process.read_addr32(ctx.engine_module.base + offset)?;

        Ok(ClientState { ptr })
    }

    pub fn get_viewangles(&self, ctx: &mut CheatCtx) -> Result<Vec3, Error> {
        let offset = ctx.offsets.get_sig("dwClientState_ViewAngles")?;
        Ok(ctx.process.read(self.ptr + offset)?)
    }

    pub fn set_viewangles(&self, ctx: &mut CheatCtx, newangles: Vec3) -> Result<(), Error> {
        let offset = ctx.offsets.get_sig("dwClientState_ViewAngles")?;
        Ok(ctx.process.write(self.ptr + offset, &newangles)?)
    }

    pub fn get_userinfo_table(
        &self,
        ctx: &mut CheatCtx,
        index: usize,
    ) -> Result<PlayerInfo, Error> {
        let offset = ctx.offsets.get_sig("dwClientState_PlayerInfo").unwrap();
        let userinfotable_ptr = ctx.process.read_addr32(self.ptr + offset)?;
        let items_ptr = ctx.process.read_addr32(userinfotable_ptr + 0x40)?;
        let items = ctx.process.read_addr32(items_ptr + 0xC)?;

        Ok(ctx.process.read(items + 0x28 + ((index - 1) * 0x34))?)
    }
}

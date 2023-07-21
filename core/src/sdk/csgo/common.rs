use memflow::{prelude::MemoryView, types::Address};

use crate::{
    cheat_ctx::CheatCtx,
    sdk::{error::Error, structs::GlobalVars},
};

pub fn get_playerresource(ctx: &mut CheatCtx) -> Result<Address, Error> {
    let offset = ctx.offsets.get_sig("dwPlayerResource")?;
    Ok(ctx.process.read_addr32(ctx.client_module.base + offset)?)
}

pub fn get_radarbase(ctx: &mut CheatCtx) -> Result<Address, Error> {
    let offset = ctx.offsets.get_sig("dwRadarBase")?;
    let offset = ctx.process.read_addr32(ctx.client_module.base + offset)?;

    Ok(ctx.process.read_addr32(offset + 0x74)?)
}

pub fn get_globalvars(ctx: &mut CheatCtx) -> Result<GlobalVars, Error> {
    let offset = ctx.offsets.get_sig("dwGlobalVars")?;
    Ok(ctx.process.read(ctx.engine_module.base + offset)?)
}

pub fn get_glowmanager(ctx: &mut CheatCtx) -> Result<Address, Error> {
    let offset = ctx.offsets.get_sig("dwGlowObjectManager")?;
    Ok(ctx.process.read_addr32(ctx.client_module.base + offset)?)
}

pub const RANKS: [&str; 19] = [
    "Unranked",
    "Silver I",
    "Silver II",
    "Silver III",
    "Silver IV",
    "Silver Elite",
    "Silver Elite Master",
    "Gold Nova I",
    "Gold Nova II",
    "Gold Nova III",
    "Gold Nova Master",
    "Master Guardian I",
    "Master Guardian II",
    "Master Guardian Elite",
    "Distinguished Master Guardian",
    "Legendary Eagle",
    "Legendary Eagle Master",
    "Supreme Master First Class",
    "The Global Elite",
];

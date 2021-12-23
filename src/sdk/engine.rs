use memflow::VirtualMemory;

use crate::core::structs::CheatCtx;

use super::structs::global_vars::GlobalVars;

pub fn get_globalvars(ctx: &mut CheatCtx) -> GlobalVars {
    let offset = ctx.offsets["dwGlobalVars"];
    ctx.process.virt_mem.virt_read(ctx.engine_module.base + offset)
        .unwrap()
}
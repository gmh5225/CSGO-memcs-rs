use memflow::{prelude::MemoryView, types::Address};

use crate::{
    cheat_ctx::CheatCtx,
    sdk::{
        error::Error,
        structs::{Matrix3x4, RadarPlayer, Vec3},
    },
};

pub struct Entity {
    pub ptr: Address,
    pub index: usize,
}

impl Entity {
    pub fn from_index(ctx: &mut CheatCtx, index: usize) -> Result<Entity, Error> {
        let offset = ctx.offsets.get_sig("dwEntityList")?;
        let ptr = ctx
            .process
            .read_addr32(ctx.client_module.base + offset + (index * 0x10))?;

        Ok(Entity { ptr, index })
    }

    pub fn get_local(ctx: &mut CheatCtx) -> Result<Entity, Error> {
        let offset = ctx.offsets.get_sig("dwLocalPlayer")?;
        let ptr = ctx.process.read_addr32(ctx.client_module.base + offset)?;

        let index: i32 = ctx.process.read(ptr + 0x64)?;

        Ok(Entity {
            ptr,
            index: index as usize,
        })
    }

    pub fn get_index(&self, ctx: &mut CheatCtx) -> Result<i32, Error> {
        Ok(ctx.process.read(self.ptr + 0x64)?)
    }

    pub fn get_health(&self, ctx: &mut CheatCtx) -> Result<i32, Error> {
        let offset = ctx.offsets.get_var("m_iHealth")?;
        Ok(ctx.process.read(self.ptr + offset)?)
    }

    pub fn get_team(&self, ctx: &mut CheatCtx) -> Result<i32, Error> {
        let offset = ctx.offsets.get_var("m_iTeamNum")?;
        Ok(ctx.process.read(self.ptr + offset)?)
    }

    pub fn get_dormant(&self, ctx: &mut CheatCtx) -> Result<bool, Error> {
        let offset = ctx.offsets.get_var("m_bDormant")?;
        let data: u8 = ctx.process.read(self.ptr + offset)?;

        Ok(data != 0)
    }

    pub fn get_glowindex(&self, ctx: &mut CheatCtx) -> Result<usize, Error> {
        let offset = ctx.offsets.get_var("m_iGlowIndex")?;
        let data: i32 = ctx.process.read(self.ptr + offset)?;

        Ok(data as usize)
    }

    pub fn get_pos(&self, ctx: &mut CheatCtx) -> Result<Vec3, Error> {
        let offset = ctx.offsets.get_var("m_vecOrigin")?;
        Ok(ctx.process.read(self.ptr + offset)?)
    }

    pub fn get_viewoffset(&self, ctx: &mut CheatCtx) -> Result<Vec3, Error> {
        let offset = ctx.offsets.get_var("m_vecViewOffset")?;
        Ok(ctx.process.read(self.ptr + offset)?)
    }

    pub fn get_aimpunch(&self, ctx: &mut CheatCtx) -> Result<Vec3, Error> {
        let offset = ctx.offsets.get_var("m_aimPunchAngle")?;
        Ok(ctx.process.read(self.ptr + offset)?)
    }

    pub fn get_spotted(&self, ctx: &mut CheatCtx) -> Result<bool, Error> {
        let offset = ctx.offsets.get_var("m_bSpotted")?;
        let data: u8 = ctx.process.read(self.ptr + offset)?;

        Ok(data != 0)
    }

    pub fn set_spotted(&self, ctx: &mut CheatCtx, value: bool) -> Result<(), Error> {
        let offset = ctx.offsets.get_var("m_bSpotted")?;
        Ok(ctx.process.write(self.ptr + offset, &(value as u8))?)
    }

    pub fn get_spotted_mask(&self, ctx: &mut CheatCtx) -> Result<i32, Error> {
        let offset = ctx.offsets.get_var("m_bSpottedByMask")?;
        Ok(ctx.process.read(self.ptr + offset)?)
    }

    pub fn get_bonematrix_ptr(&self, ctx: &mut CheatCtx) -> Result<Address, Error> {
        let offset = ctx.offsets.get_var("m_dwBoneMatrix")?;
        Ok(ctx.process.read_addr32(self.ptr + offset)?)
    }

    pub fn get_bonepos(&self, ctx: &mut CheatCtx, bone_id: usize) -> Result<Vec3, Error> {
        let bonematrix_ptr = self.get_bonematrix_ptr(ctx)?;
        let bonematrix: Matrix3x4 = ctx.process.read(bonematrix_ptr + (bone_id * 0x30))?;

        Ok(Vec3::new(
            bonematrix.row0[3],
            bonematrix.row1[3],
            bonematrix.row2[3],
        ))
    }

    pub fn get_comp_rank(
        &self,
        ctx: &mut CheatCtx,
        player_resources: Address,
    ) -> Result<i32, Error> {
        let offset = ctx.offsets.get_var("m_iCompetitiveRanking")?;
        Ok(ctx
            .process
            .read(player_resources + offset + (self.index * 0x4))?)
    }

    pub fn get_comp_wins(
        &self,
        ctx: &mut CheatCtx,
        player_resources: Address,
    ) -> Result<i32, Error> {
        let offset = ctx.offsets.get_var("m_iCompetitiveWins")?;
        Ok(ctx
            .process
            .read(player_resources + offset + (self.index * 0x4))?)
    }

    pub fn get_radarplayer(
        &self,
        ctx: &mut CheatCtx,
        radar_base: Address,
    ) -> Result<RadarPlayer, Error> {
        Ok(ctx
            .process
            .read(radar_base + (0x174 * (self.index + 1)) - 0x3C)?)
    }

    pub fn get_entity_list(&self, ctx: &mut CheatCtx) -> Result<Address, Error> {
        let offset = ctx.offsets.get_sig("dwEntityList")?;
        Ok(ctx
            .process
            .read_addr32(ctx.client_module.base + offset + (self.index * 0x10))?)
    }

    pub fn get_class_id(&self, ctx: &mut CheatCtx) -> Result<u32, Error> {
        let ptr1 = ctx.process.read_addr32(self.ptr + 0x8)?;
        let ptr2 = ctx.process.read_addr32(ptr1 + 2 * 0x4)?;
        let ptr3 = ctx.process.read_addr32(ptr2 + 0x1)?;
        Ok(ctx.process.read(ptr3 + 0x14)?)
    }

    pub fn is_player(&self, ctx: &mut CheatCtx) -> Result<bool, Error> {
        let class_id = self.get_class_id(ctx)?;
        log::info!("{}", class_id);
        Ok(class_id == 40)
    }
}

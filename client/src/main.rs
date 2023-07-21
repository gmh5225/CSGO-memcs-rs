use std::time::Duration;

use memflow::prelude::MemoryView;

use memcs_core::{sdk::{csgo::Entity, self}, CheatCtx};

fn glow(ctx: &mut CheatCtx) {
    let glowmanager = sdk::csgo::get_glowmanager(ctx).unwrap();
    let local = Entity::get_local(ctx).unwrap();
    let local_team = local.get_team(ctx).unwrap();
    let globals = sdk::csgo::get_globalvars(ctx).unwrap();

    for index in 0..globals.max_clients {
        let entity = Entity::from_index(ctx, index as usize).unwrap();

        if entity.get_health(ctx).unwrap() <= 0 {
            continue;
        }

        if entity.get_team(ctx).unwrap() != local_team {
            let glow_idx = entity.get_glowindex(ctx).unwrap();

            // write glow shit
            let offset = glow_idx * 0x38;

            let color = sdk::structs::GlowObjectColor {
                channel_r: 1.0,
                channel_g: 0.25,
                channel_b: 0.25,
                channel_a: 0.75
            };

            let occlusion = sdk::structs::GlowObjectOcclusion {
                render_when_occluded: true,
                render_when_unoccluded: false,
            };

            ctx.process.write(glowmanager + (offset + 0x8), &color).unwrap();
            ctx.process.write(glowmanager + (offset + 0x28), &occlusion).unwrap();
        }
    }
}


fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    let mut ctx = memcs_core::CheatCtx::setup().unwrap();

    println!("Initialized cheat...");
    println!("Running loop!");

    loop {
        glow(&mut ctx);

        std::thread::sleep(Duration::from_millis(1))
    }
}

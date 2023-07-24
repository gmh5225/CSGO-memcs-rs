mod structs;
mod menu;

use std::{time::Duration, sync::{Mutex, Arc}, thread::JoinHandle};

use memflow::{prelude::MemoryView, types::Address};

use memcs_core::{sdk::{csgo::{Entity, set_model_brightness, SignOnState, set_send_packet, get_attack}, self}, CheatCtx};
use simple_file_logger::{init_logger, LogLevel};
use structs::{SharedData, Color, PlayerInfo};

fn chams(ctx: &mut CheatCtx, entity: &Entity, color: Color) {
    let entity_list = entity.get_entity_list(ctx).unwrap();

    ctx.process.write(entity_list + 0x70, &color.red()).unwrap();
    ctx.process.write(entity_list + 0x71, &color.green()).unwrap();
    ctx.process.write(entity_list + 0x72, &color.blue()).unwrap();
}

fn glow(ctx: &mut CheatCtx, glowmanager: Address, entity: &Entity) {
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

fn cheat_thread(shared_data: Arc<Mutex<SharedData>>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut ctx = memcs_core::CheatCtx::setup().unwrap();
        let chams_color = Color::new(25, 255, 25, 50.0);

        loop {
            let start_time = std::time::Instant::now();

            let client_state = sdk::csgo::ClientState::get(&mut ctx).unwrap();
            let game_state = client_state.get_state(&mut ctx).unwrap();

            if game_state == SignOnState::Full {
                let glowmanager = sdk::csgo::get_glowmanager(&mut ctx).unwrap();
                let local = Entity::get_local(&mut ctx).unwrap();
                let local_index = local.get_index(&mut ctx).unwrap();
                let local_team = local.get_team(&mut ctx).unwrap();
                let globals = sdk::csgo::get_globalvars(&mut ctx).unwrap();
                let mut names = vec![PlayerInfo::default(); 64];

                if shared_data.lock().unwrap().config.fakelag {
                    if globals.tickcount % 16 == 0 {
                        set_send_packet(&mut ctx, true).unwrap();
                    } else {
                        set_send_packet(&mut ctx, false).unwrap();
                    }
                }
    
                for index in 0..globals.max_clients {
                    let entity = Entity::from_index(&mut ctx, index as usize).unwrap();
    
                    if !entity.is_player(&mut ctx).unwrap() {
                        continue;
                    }
    
                    let mut is_alive = true;
            
                    if entity.get_health(&mut ctx).unwrap() <= 0 {
                        is_alive = false;
                    }
    
                    let is_enemy = entity.get_team(&mut ctx).unwrap() != local_team;
                    let is_local = local_index - 1 == index;
                    let player_info = client_state.get_userinfo_table(&mut ctx, index).unwrap();
                    let name = player_info.get_name(&mut ctx).unwrap();
                    names[index as usize] = PlayerInfo::new(Some(name), is_enemy, is_local, is_alive);
    
                    if !is_alive {
                        continue;
                    }
    
                    if is_enemy {
                        let lock = shared_data.lock().unwrap();
    
                        if lock.config.glow {
                            glow(&mut ctx, glowmanager, &entity);
                        }
    
                        if lock.config.radar {
                            entity.set_spotted(&mut ctx, true).unwrap();
                        }
    
                        if lock.config.chams {
                            chams(&mut ctx, &entity, chams_color);
                        } else if lock.chams_once {
                            chams(&mut ctx, &entity, Color::new(255, 255, 255, 0.0));
                        }
                    }
                }
    
                let mut lock = shared_data.lock().unwrap();
                lock.player_list = names.to_vec();
    
                if lock.config.chams {
                    set_model_brightness(&mut ctx, chams_color.brightness()).unwrap();
    
                    if !lock.chams_once {
                        lock.chams_once = true;
                    }
                } else if lock.chams_once {
                    set_model_brightness(&mut ctx, 0.0).unwrap();
                    lock.chams_once = false;
                }

                if lock.config.fakelag {
                    lock.fakelag_once = true;
                } else if lock.fakelag_once {
                    set_send_packet(&mut ctx, true).unwrap();
                    lock.fakelag_once = false;
                }
            } else {
                let mut lock = shared_data.lock().unwrap();

                if lock.chams_once {
                    set_model_brightness(&mut ctx, 0.0).unwrap();
                    lock.chams_once = false;
                }

                lock.player_list = vec![];
            }

            let end_time = std::time::Instant::now();
            let elapsed = end_time.checked_duration_since(start_time);
    
            let mut lock = shared_data.lock()
                .expect("Mutex is poisoned!");

            lock.elapsed_time = elapsed;
            lock.game_state = Some(game_state);

            if lock.engine_base.is_none() {
                lock.engine_base = Some(ctx.engine_module.base);
            }

            if lock.client_base.is_none() {
                lock.client_base = Some(ctx.client_module.base);
            }

            if lock.should_exit {
                if !lock.chams_once && !lock.fakelag_once {
                    break;
                } else {
                    lock.config.chams = false;
                    lock.config.fakelag = false;
                }
            }

            drop(lock);
            
            std::thread::sleep(Duration::from_millis(1))
        }
    })
}

fn main() {
    init_logger("memcs-client", LogLevel::Info).unwrap();

    let data = SharedData::default();
    let shared_data = Arc::new(Mutex::new(data));

    let join_handle = cheat_thread(Arc::clone(&shared_data));
    menu::run_menu(Arc::clone(&shared_data)).unwrap();
    join_handle.join().expect("Cheat is still running.");
}

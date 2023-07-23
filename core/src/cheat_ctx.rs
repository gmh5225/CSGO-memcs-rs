use log::info;
use memflow::prelude::v1::*;

use crate::sdk::{self, Offsets};

pub struct CheatCtx {
    pub process: IntoProcessInstanceArcBox<'static>,
    pub client_module: ModuleInfo,
    pub engine_module: ModuleInfo,
    pub offsets: Offsets,
}

impl CheatCtx {
    pub fn setup() -> std::result::Result<CheatCtx, super::sdk::error::Error> {
        let inventory = Inventory::scan();

        let os = inventory.builder()
            .connector("qemu")
            .os("win32")
            .build()?;

        info!("Created kernel - addr: {}", os.info().base);

        let mut process = os.into_process_by_name("csgo.exe")?;

        info!(
            "Found csgo process - {} - addr: {}",
            process.info().pid,
            process.info().address
        );

        let client_module = process.module_by_name("client.dll")?;
        info!("Found client module - addr: {}", client_module.base);

        let engine_module = process.module_by_name("engine.dll")?;
        info!("Found engine module - addr: {}", engine_module.base);

        let offsets = sdk::get_offsets()?;

        let ctx = Self {
            process,
            client_module,
            engine_module,
            offsets,
        };

        Ok(ctx)
    }
}

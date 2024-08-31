use thiserror::Error;

use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::DataRead;
use xplm::debugln;
use xplm::flight_loop::FlightLoop;
use xplm::plugin::{Plugin, PluginInfo};

use super::flight_loop::FlightLoopHandler;
use super::loadout::Data;

pub static NAME: &str = concat!("Persistent Loadout", " ", "v", env!("CARGO_PKG_VERSION"));
static SIGNATURE: &str = concat!("com.x-plane.xplm.", env!("CARGO_PKG_NAME"));
static DESCRIPTION: &str = "Persistent loadout for the FlyJSim Dash 8 Q4XP";

pub static DATA_FILE_PATH: &str = "Output/Q4XP/persistent-loadout.json";

#[derive(Error, Debug)]
pub enum PluginError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FindDataRef(#[from] FindError),
    #[error("no cold and dark startup")]
    NoColdAndDarkStartup,
}

pub struct PersistentLoadoutPlugin {
    handler: FlightLoop,
}

impl Plugin for PersistentLoadoutPlugin {
    type Error = PluginError;

    fn start() -> Result<Self, Self::Error> {
        debugln!("{NAME} starting up...");

        let plugin = Self {
            handler: FlightLoop::new(FlightLoopHandler),
        };

        Ok(plugin)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from(NAME),
            signature: String::from(SIGNATURE),
            description: String::from(DESCRIPTION),
        }
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        let startup_running: DataRef<i32> = DataRef::find("sim/operation/prefs/startup_running")?;
        if startup_running.get() != 0 {
            return Err(PluginError::NoColdAndDarkStartup);
        }

        debugln!("{NAME} enabled...");
        self.handler.schedule_after_loops(60);

        Ok(())
    }

    fn disable(&mut self) {
        if let Err(e) = Data::save_aircraft_loadout() {
            debugln!("{NAME} {e}");
        }

        self.handler.deactivate();
        debugln!("{NAME} disabled...");
    }
}

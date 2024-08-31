use xplm::debugln;
use xplm::flight_loop::FlightLoopCallback;

use super::loadout::Data;
use super::plugin::NAME;

pub struct FlightLoopHandler;

impl FlightLoopCallback for FlightLoopHandler {
    fn flight_loop(&mut self, state: &mut xplm::flight_loop::LoopState) {
        if let Err(e) = Data::restore_aircraft_loadout() {
            debugln!("{NAME} {e}");
        }

        // We are done...
        state.deactivate();
    }
}

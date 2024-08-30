use xplm::debugln;
use xplm::flight_loop::FlightLoopCallback;

use super::loadout::Data;
use super::plugin::NAME;

pub struct FlightLoopHandler;

impl FlightLoopCallback for FlightLoopHandler {
    fn flight_loop(&mut self, state: &mut xplm::flight_loop::LoopState) {
        let data = match Data::from_file("Output/Q4XP/persistent-loadout.json") {
            Ok(d) => d,
            Err(e) => {
                debugln!("{NAME} {e}");
                state.deactivate();
                return;
            }
        };

        if let Err(e) = data.write_into_sim() {
            debugln!("{NAME} {e}");
            state.deactivate();
            return;
        }

        // We are done...
        state.deactivate();
    }
}

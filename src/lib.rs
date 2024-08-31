mod datarefs;
mod flight_loop;
mod loadout;
mod plugin;

use xplm::xplane_plugin;

pub use plugin::PersistentLoadoutPlugin;

xplane_plugin!(PersistentLoadoutPlugin);

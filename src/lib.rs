use xplm::xplane_plugin;

mod data;
mod flight_loop;
mod loadout;
mod plugin;

use plugin::PersistentLoadoutPlugin;

xplane_plugin!(PersistentLoadoutPlugin);

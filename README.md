# Persistent Loadout Plugin for the FlyJSim Q4XP
This plugin saves and restores the fuel tank levels on leaving the aircraft and entering the aircraft in cold and dark.

The fuel tank levels are saved on a per livery basis in a JSON file in the following X-Plane 12 location: `Output/Q4XP/persistent-loadout.json`

## Installation
To install, download the latest [release](https://github.com/telephono/q4xp-persistent-loadout/releases), extract and copy the `persistent-loadout` folder to the Q4XP's `plugins` directory.

## Known Issues
The JPAD will always show 1329 kg of fuel on startup, regardless of the actual fuel levels. Which makes this plugin kind of pointless, I guess?

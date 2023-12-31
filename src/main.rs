//#![windows_subsystem = "windows"]

use std::{fs, thread, time::Duration};

use auras::{delay_display, lua_aura, nelth_aura::NelthAura};
use log_watchers::LogDispatcher;
mod auras;
mod log_watchers;
mod main_window;
mod utils;

fn main() {
    env_logger::init();
    procspawn::init();

    main_window::App::spawn();
    let path = main_window::LOG_FILE.lock().clone();
    let replay = *main_window::REPLAY.lock();
    let player_list = main_window::PLAYER_LIST.lock().clone();
    //let (nelth_channel_tx, nelth_channel_rx) = unbounded();
    let (_nelth_handle, nelth_tx) = NelthAura::spawn(player_list);
    let (_delay_handle, delay_tx) = delay_display::Aura::spawn();
    let mut aura_channels = vec![nelth_tx, delay_tx];
    if let Ok(script) = fs::read_to_string("lua-auras/aura.lua") {
        let (_lua_handle, lua_tx) = lua_aura::Aura::spawn(script);
        aura_channels.push(lua_tx);
    }
    let _log_dispatcher = LogDispatcher::new(path, replay, aura_channels);

    // Keep alive, until we get a proper menu that handles it instead
    loop {
        thread::sleep(Duration::from_secs(10));
    }
}

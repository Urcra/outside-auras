#![windows_subsystem = "windows"]

use auras::nelth_aura::NelthAura;
use crossbeam::channel::unbounded;
use log_watchers::LogDispatcher;
mod auras;
mod log_watchers;
mod main_window;
mod utils;

fn main() {
    env_logger::init();
    main_window::App::spawn();
    let path = main_window::LOG_FILE.lock().clone();
    let replay = *main_window::REPLAY.lock();
    let player_list = main_window::PLAYER_LIST.lock().clone();
    let (nelth_channel_tx, nelth_channel_rx) = unbounded();
    let _log_dispatcher = LogDispatcher::new(path, replay, vec![nelth_channel_tx]);

    NelthAura::spawn(nelth_channel_rx, player_list);
}

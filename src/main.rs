use auras::nelth_aura::NelthAura;
use crossbeam::channel::unbounded;
use eframe::egui;
use log_watchers::LogDispatcher;
use std::env;
mod auras;
mod log_watchers;
mod utils;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        // 527.0, 454.0
        initial_window_size: Some(egui::vec2(127.0, 154.0)),
        decorated: false,
        always_on_top: true,
        mouse_passthrough: true,
        transparent: false,
        ..Default::default()
    };
    let path = env::args().nth(1).unwrap();
    let replay_or_watch = env::args().nth(2).unwrap();
    let (nelth_channel_tx, nelth_channel_rx) = unbounded();
    let _log_dispatcher =
        LogDispatcher::new(path, replay_or_watch == "replay", vec![nelth_channel_tx]);
    eframe::run_native(
        "Outside Auras",
        options,
        Box::new(|cc| Box::new(NelthAura::new(cc, nelth_channel_rx))),
    )
}

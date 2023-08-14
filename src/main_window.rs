use lazy_static::lazy_static;

use eframe::egui;
use parking_lot::Mutex;

pub struct App {}

lazy_static! {
    pub static ref REPLAY: Mutex<bool> = Mutex::new(false);
    pub static ref PLAYER_LIST: Mutex<String> = Mutex::new(String::new());
    pub static ref LOG_FILE: Mutex<String> = Mutex::new(String::new());
}

impl App {
    pub fn spawn() {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(325.0, 250.0)),
            run_and_return: true,
            ..Default::default()
        };
        eframe::run_native("Outside Auras", options, Box::new(|_cc| Box::new(App {}))).unwrap();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _window_frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("OutsideAuras");
            ui.label("Enter the full path to logfile");
            let mut logfile = LOG_FILE.lock().clone();
            if egui::TextEdit::singleline(&mut logfile).hint_text("C:\\Program Files (x86)\\World of Warcraft\\_retail_\\Logs\\WoWCombatLog-081223_203440").show(ui).response.changed() {
                *LOG_FILE.lock() = logfile.clone();
            }
            ui.label("Enter players for custom sorting");
            let mut player_list = PLAYER_LIST.lock().clone();
            if egui::TextEdit::multiline(&mut player_list).hint_text("Dratnos\nTettles\nMagePlayer\n..").show(ui).response.changed() {
                *PLAYER_LIST.lock() = player_list.clone();
            }
            let mut replay_selected = REPLAY.lock().clone();
            if ui.checkbox(&mut replay_selected, "Replay old log").changed() {
                *REPLAY.lock() = replay_selected;
            }
            if ui.button("Start auras").clicked() {
                _window_frame.close();
            }
        });
    }
}

use std::{sync::Arc, thread};

use crossbeam::channel::Receiver;
use eframe::{
    egui::{self},
    emath::Align2,
    epaint::{Color32, FontId, TextureHandle, Vec2},
};
use parking_lot::Mutex;

use crate::utils::load_image_from_path;

use super::{list_aura::ListAura, AuraBackground, AuraIcon, AuraItem, AuraText};

pub struct NelthAura {
    shared_state: Arc<Mutex<NelthSharedState>>,
    icon: TextureHandle,
    player_list: Vec<String>,
}

struct NelthSharedState {
    volanic_hearts: Vec<String>,
    ctx: egui::Context,
}

impl ListAura for NelthAura {
    fn required_size(&self) -> Vec2 {
        let height = self.shared_state.lock().volanic_hearts.len() as f32 * self.item_height();
        let width = self.item_width();
        Vec2 {
            x: width,
            y: height,
        }
    }

    fn item_height(&self) -> f32 {
        56.0
    }

    fn item_width(&self) -> f32 {
        256.0
    }

    fn items(&self) -> Vec<AuraItem> {
        let mut items = Vec::new();

        let icon_text_font = FontId::monospace(40.0);
        let icon_text_color = Color32::WHITE;
        let icon_text_align = Align2::CENTER_CENTER;
        let icon_text_offset = 56.0 / 2.0;

        let text_font = FontId::monospace(25.0);
        let text_color = Color32::WHITE;
        let text_align = Align2::LEFT_CENTER;
        let text_y_offset = 56.0 / 2.0;
        let text_x_offset = 66.0;

        let background = AuraBackground {
            color: Color32::DARK_GRAY,
            width: 256.0,
            height: 56.0,
        };

        let mut hearts = self.shared_state.lock().volanic_hearts.clone();
        hearts.sort_by_key(|v| self.player_list.iter().position(|p| p == v).unwrap_or(0));
        for (i, player_name) in hearts.iter().enumerate() {
            let icon_text = AuraText {
                content: (i + 1).to_string(),
                anchor: icon_text_align,
                font: icon_text_font.clone(),
                color: icon_text_color,
                x_offset: icon_text_offset,
                y_offset: icon_text_offset,
            };

            let text = AuraText {
                content: player_name.clone(),
                anchor: text_align,
                font: text_font.clone(),
                color: text_color,
                x_offset: text_x_offset,
                y_offset: text_y_offset,
            };

            let item = AuraItem {
                icon: Some(self.icon()),
                icon_text: Some(icon_text),
                text: Some(text),
                background: Some(background),
            };
            items.push(item);
        }
        items
    }
}

impl NelthAura {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        log_receiver: Receiver<String>,
        player_list: Vec<String>,
    ) -> Self {
        let ctx = cc.egui_ctx.clone();
        let image = load_image_from_path("res/inv_wildfirebomb_blood.jpg").unwrap();
        let icon = ctx.load_texture("volcanic-heartbeat", image, Default::default());
        let shared_state = Arc::new(Mutex::new(NelthSharedState {
            volanic_hearts: vec![],
            ctx,
        }));
        let state_clone = shared_state.clone();
        thread::spawn(move || loop {
            match log_receiver.recv() {
                Ok(s) => Self::handle_log_line(&state_clone, &s),
                Err(e) => println!("Error {e}"),
            }
        });

        Self {
            shared_state,
            icon,
            player_list,
        }
    }

    pub fn spawn(log_receiver: Receiver<String>, player_list: String) {
        let options = eframe::NativeOptions {
            // 527.0, 454.0
            initial_window_size: Some(egui::vec2(127.0, 154.0)),
            decorated: false,
            always_on_top: true,
            mouse_passthrough: false,
            transparent: false,
            ..Default::default()
        };
        let mut player_list_fixed = Vec::new();
        for player in player_list.lines() {
            let player = player.trim().to_string();
            if !player.is_empty() {
                player_list_fixed.push(player);
            }
        }
        eframe::run_native(
            "Outside Auras",
            options,
            Box::new(|cc| Box::new(NelthAura::new(cc, log_receiver, player_list_fixed))),
        )
        .unwrap();
    }

    fn handle_log_line(state: &Arc<Mutex<NelthSharedState>>, line: &str) {
        let mut split = line.split("  ");
        let _date_time = split.next().unwrap().trim();
        let csv = split.next().unwrap();

        let mut csv = csv.split(",");
        let event_type = csv.next().unwrap();

        match event_type {
            "ENCOUNTER_START" | "ENCOUNTER_END" => {
                println!("Hey");
                state.lock().volanic_hearts.clear();
            }
            "SPELL_AURA_APPLIED" => {
                let _caster_guid = csv.next().unwrap();
                let _caster_name = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _target_guid = csv.next().unwrap();
                let target_name = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let spell_name = csv.next().unwrap();
                if spell_name == "\"Volcanic Heartbeat\"" {
                    println!("Heyo");
                    let name = target_name.split_once("-").unwrap().0[1..].to_string();
                    state.lock().volanic_hearts.push(name);
                    state.lock().ctx.request_repaint();
                }
            }
            "SPELL_AURA_REMOVED" => {
                let _caster_guid = csv.next().unwrap();
                let _caster_name = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _target_guid = csv.next().unwrap();
                let target_name = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let _ = csv.next().unwrap();
                let spell_name = csv.next().unwrap();
                if spell_name == "\"Volcanic Heartbeat\"" {
                    println!("Heya");
                    let name = &target_name.split_once("-").unwrap().0[1..].to_string();
                    state.lock().volanic_hearts.retain(|e| e != name);
                    state.lock().ctx.request_repaint();
                }
            }
            _ => {}
        }
    }

    fn icon(&self) -> AuraIcon {
        AuraIcon {
            image: &self.icon,
            height: 56.0,
            width: 56.0,
        }
    }
}

impl eframe::App for NelthAura {
    fn update(&mut self, ctx: &egui::Context, window_frame: &mut eframe::Frame) {
        self.eframe_update(ctx, window_frame)
    }
}

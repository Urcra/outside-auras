use std::{sync::Arc, thread};

use eframe::{
    egui::{self},
    emath::Align2,
    epaint::{Color32, FontId, TextureHandle, Vec2},
};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcReceiver, IpcSender};
use parking_lot::Mutex;
use procspawn::JoinHandle;
use rlua::{Function, Lua};

use crate::utils::load_image_from_path;

use super::{list_aura::ListAura, AuraBackground, AuraIcon, AuraItem, AuraText};

pub struct Aura {
    shared_state: Arc<Mutex<SharedState>>,
    icon: TextureHandle,
}

struct SharedState {
    ctx: egui::Context,
    lua: Lua,
}

impl ListAura for Aura {
    fn required_size(&self) -> Vec2 {
        self.shared_state.lock().lua.context(|ctx| {
            let globals = ctx.globals();
            let required_size_x_fun: Function = globals.get("required_height").unwrap();
            let size_y = required_size_x_fun.call::<(), f32>(()).unwrap();
            let required_size_y_fun: Function = globals.get("required_width").unwrap();
            let size_x = required_size_y_fun.call::<(), f32>(()).unwrap();
            Vec2 {
                x: size_x,
                y: size_y,
            }
        })
    }

    fn item_height(&self) -> f32 {
        self.shared_state.lock().lua.context(|ctx| {
            let globals = ctx.globals();
            let item_height_fun: Function = globals.get("item_height").unwrap();
            item_height_fun.call::<(), f32>(()).unwrap()
        })
    }

    fn item_width(&self) -> f32 {
        self.shared_state.lock().lua.context(|ctx| {
            let globals = ctx.globals();
            let item_width_fun: Function = globals.get("item_width").unwrap();
            item_width_fun.call::<(), f32>(()).unwrap()
        })
    }

    fn items(&self) -> Vec<AuraItem> {
        let lua_items = self.shared_state.lock().lua.context(|ctx| {
            let globals = ctx.globals();
            let items_fun: Function = globals.get("items").unwrap();
            items_fun.call::<(), Vec<Vec<String>>>(()).unwrap()
        });

        let lua_items: Vec<(String, String)> = lua_items
            .iter()
            .map(|e| (e[0].clone(), e[1].clone()))
            .collect();

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

        let mut items = vec![];

        for (icon_string, content_string) in lua_items.iter() {
            let icon_text = AuraText {
                content: icon_string.clone(),
                anchor: icon_text_align,
                font: icon_text_font.clone(),
                color: icon_text_color,
                x_offset: icon_text_offset,
                y_offset: icon_text_offset,
            };

            let text = AuraText {
                content: content_string.clone(),
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

impl Aura {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        log_receiver: IpcReceiver<String>,
        lua_script: String,
    ) -> Self {
        let lua = Lua::new();
        lua.context(|ctx| {
            let globals = ctx.globals();
            let table = ctx.create_table().unwrap();
            globals.set("aura_env", table).unwrap();
            ctx.load(&lua_script).exec().unwrap();
        });

        let ctx = cc.egui_ctx.clone();
        let image = load_image_from_path("res/lua_icon.jpg").unwrap();
        let icon = ctx.load_texture("lua-icon", image, Default::default());
        let shared_state = Arc::new(Mutex::new(SharedState { ctx, lua }));
        let state_clone = shared_state.clone();
        thread::spawn(move || loop {
            match log_receiver.recv() {
                Ok(s) => Self::handle_log_line(&state_clone, &s),
                Err(e) => println!("Error receiving log: {e}"),
            }
        });

        Self { shared_state, icon }
    }

    //TODO: make this more generic
    pub fn spawn(lua_script: String) -> (JoinHandle<()>, IpcSender<String>) {
        let (server, server_name) = IpcOneShotServer::new().unwrap();
        let handle = procspawn::spawn((server_name, lua_script), |(server_name, lua_script)| {
            let (tx, log_receiver) = ipc::channel::<String>().unwrap();
            let tx0 = IpcSender::connect(server_name).unwrap();
            tx0.send(tx).unwrap();
            let options = eframe::NativeOptions {
                // 527.0, 454.0
                initial_window_size: Some(egui::vec2(127.0, 154.0)),
                decorated: false,
                always_on_top: true,
                mouse_passthrough: false,
                transparent: false,
                ..Default::default()
            };
            eframe::run_native(
                "Lua aura",
                options,
                Box::new(|cc| Box::new(Aura::new(cc, log_receiver, lua_script))),
            )
            .unwrap();
        });
        let (_, log_sender): (_, IpcSender<String>) = server.accept().unwrap();
        (handle, log_sender)
    }

    fn handle_log_line(state: &Arc<Mutex<SharedState>>, line: &str) {
        let line = line.to_string();

        let should_redraw = state.lock().lua.context(|ctx| {
            let globals = ctx.globals();
            let handle_log_line_fun: Function = globals.get("handle_log_line").unwrap();
            handle_log_line_fun.call::<String, bool>(line).unwrap()
        });

        if should_redraw {
            state.lock().ctx.request_repaint();
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

impl eframe::App for Aura {
    fn update(&mut self, ctx: &egui::Context, window_frame: &mut eframe::Frame) {
        window_frame.drag_window();
        self.eframe_update(ctx, window_frame)
    }
}

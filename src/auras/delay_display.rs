use std::{sync::Arc, thread, time::Instant};

use eframe::egui;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcReceiver, IpcSender};
use parking_lot::Mutex;
use procspawn::JoinHandle;

pub struct Aura {
    shared_state: Arc<Mutex<State>>,
}

struct State {
    last_event: Instant,
    last_delay: u32,
    ctx: egui::Context,
}

impl Aura {
    pub fn new(cc: &eframe::CreationContext<'_>, log_receiver: IpcReceiver<String>) -> Self {
        let ctx = cc.egui_ctx.clone();
        let shared_state = Arc::new(Mutex::new(State {
            last_event: Instant::now(),
            last_delay: 0,
            ctx,
        }));
        let state_clone = shared_state.clone();
        thread::spawn(move || loop {
            match log_receiver.recv() {
                Ok(s) => Self::handle_log_line(&state_clone, &s),
                Err(e) => println!("Error receiving log: {e}"),
            }
        });

        Self { shared_state }
    }

    //TODO: make this more generic
    pub fn spawn() -> (JoinHandle<()>, IpcSender<String>) {
        let (server, server_name) = IpcOneShotServer::new().unwrap();
        let handle = procspawn::spawn(server_name, |server_name| {
            let (tx, log_receiver) = ipc::channel::<String>().unwrap();
            let tx0 = IpcSender::connect(server_name).unwrap();
            tx0.send(tx).unwrap();
            let options = eframe::NativeOptions {
                initial_window_size: Some(egui::vec2(127.0, 25.0)),
                decorated: false,
                always_on_top: true,
                mouse_passthrough: false,
                transparent: false,
                ..Default::default()
            };
            eframe::run_native(
                "Delay display",
                options,
                Box::new(|cc| Box::new(Aura::new(cc, log_receiver))),
            )
            .unwrap();
        });
        let (_, log_sender): (_, IpcSender<String>) = server.accept().unwrap();
        (handle, log_sender)
    }

    fn handle_log_line(state: &Arc<Mutex<State>>, _line: &str) {
        let mut state = state.lock();
        let delay = state.last_event.elapsed().as_millis() as u32;
        state.last_event = Instant::now();
        if delay > 10 {
            state.last_delay = delay;
            state.ctx.request_repaint();
        }
    }
}

impl eframe::App for Aura {
    fn update(&mut self, ctx: &egui::Context, _window_frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Current delay: {}",
                self.shared_state.lock().last_delay
            ));
            _window_frame.drag_window();
        });
    }
}

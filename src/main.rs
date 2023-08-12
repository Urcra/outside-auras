use std::{io::{Seek, BufReader, BufRead, SeekFrom, self}, env, fs::File, sync::{mpsc, Arc, Mutex}, time::Instant, thread};
use chrono::NaiveDateTime;
use eframe::{egui::{self, Visuals}, epaint::{TextureHandle, Pos2, Rect, Color32, Shadow, Rounding, FontId}, emath::Align2};
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config};
use std::time::Duration;


fn file_watcher(path: String, state: Arc<Mutex<SharedState>>) -> notify::Result<()> {
    // get file
    println!("Started with {:?}", path);
    // get pos to end of file
    let mut f = File::open(&path)?;
    let mut pos = f.metadata()?.len();

    // set up watcher
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    let mut last_event = Instant::now();

    state.lock().unwrap().ctx.request_repaint();

    // watch
    for res in rx {
        match res {
            Ok(_event) => {
                println!("ms since last event: {}", last_event.elapsed().as_millis());
                state.lock().unwrap().last_log_line_delay = last_event.elapsed().as_millis() as u32;
                last_event = Instant::now();
                // ignore any event that didn't change the pos
                if f.metadata()?.len() == pos {
                    continue;
                }

                // read from pos to end of file
                f.seek(SeekFrom::Start(pos + 1))?;

                // update post to end of file
                let tmp_pos = f.metadata()?.len();

                let reader = BufReader::new(&f);
                for line in reader.lines() {
                    let line = line.unwrap();
                    pos = tmp_pos;

                    let mut split = line.split("  ");
                    let _date_time = split.next().unwrap().trim();
                    let csv = split.next().unwrap();
                    //println!("{date_time}");
                    handle_line(&state, csv);
                    
                }
                
                state.lock().unwrap().ctx.request_repaint();
            }
            Err(error) => println!("{error:?}"),
        }
    }

    Ok(())
}

fn file_replayer(path: String, state: Arc<Mutex<SharedState>>) -> io::Result<()> {
    // get file
    println!("Started with {:?}", path);
    // get pos to end of file
    let f = File::open(&path)?;

    let line_reader = BufReader::new(&f);

    let mut time_of_last: Option<NaiveDateTime> = None;

    let mut last_event = Instant::now();

    for line in line_reader.lines() {
        let line = line?;
        let mut split = line.split("  ");
        let date_time = split.next().unwrap().trim();
        let date_time = format!("2023/{date_time}");
        let csv = split.next().unwrap();
        let date_time = NaiveDateTime::parse_from_str(&date_time, "%Y/%m/%d %X.%3f").unwrap();
        handle_line(&state, csv);
        match time_of_last {
            Some(last_time) => {
                let time_chunk = date_time.signed_duration_since(last_time).to_std().unwrap();
                if time_chunk > Duration::from_millis(300) {
                    {
                        let mut state = state.lock().unwrap();
                        state.last_log_line_delay = last_event.elapsed().as_millis() as u32;
                        last_event = Instant::now();
                        state.ctx.request_repaint();
                    }
                    thread::sleep(time_chunk);
                    time_of_last = Some(date_time);
                } else {

                }
            }
            None => {
                time_of_last = Some(date_time);
            }
        };
    }

    state.lock().unwrap().ctx.request_repaint();

    Ok(())
    
}


fn handle_line(state: &Arc<Mutex<SharedState>>, csv: &str) {
    let mut csv = csv.split(",");
    let event_type = csv.next().unwrap();
    

    match event_type {
        "ENCOUNTER_START" | "ENCOUNTER_END" => {
            state.lock().unwrap().volanic_hearts.clear();
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
                let name = target_name.split_once("-").unwrap().0[1..].to_string();
                state.lock().unwrap().volanic_hearts.push(name);
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
                let name = &target_name.split_once("-").unwrap().0[1..].to_string();
                state.lock().unwrap().volanic_hearts.retain(|e| e != name);
            }
        }
        _ => {}
    }
}




fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(527.0, 454.0)),
        decorated: false,
        always_on_top: true,
        mouse_passthrough: true,
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(
        "Outside Auras",
        options,
        Box::new(|cc| Box::new(NelthAura::new(cc))),
    )
}

struct NelthAura {
    shared_state: Arc<Mutex<SharedState>>,
    nelth_lair: Option<TextureHandle>,
}

struct SharedState {
    last_log_line_delay: u32,
    volanic_hearts: Vec<String>,
    ctx: egui::Context,
}

impl NelthAura {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let path = env::args().nth(1).unwrap();
        let replay_or_watch = env::args().nth(2).unwrap();
        let last_log_line_delay = 0;
        let ctx = cc.egui_ctx.clone();
        let shared_state = Arc::new(Mutex::new(SharedState {
            last_log_line_delay,
            volanic_hearts: vec![],
            ctx,
        }));
        let state_clone = shared_state.clone();
        thread::spawn(move || {
            if replay_or_watch == "replay" {
                file_replayer(path, state_clone).unwrap();
            } else if replay_or_watch == "watch" {
                file_watcher(path, state_clone).unwrap();
            } else {
                panic!("Unexpected option")
            }
            
        });
        Self {
            shared_state,
            nelth_lair: None,
        }
    }
}

impl eframe::App for NelthAura {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.clear_color(&Visuals::default());

        let my_frame = egui::containers::Frame::none().fill(Color32::TRANSPARENT).shadow(Shadow::NONE);

        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {

            let map: &egui::TextureHandle = self.nelth_lair.get_or_insert_with(|| {
                // Load the texture only once.
                let image = load_image_from_path("inv_wildfirebomb_blood.jpg").unwrap();
                ui.ctx().load_texture(
                    "volcanic-heartbeat",
                    image,
                    Default::default()
                )
            });
            let state = self.shared_state.lock().unwrap();

            let dimensions = Rect{min: Pos2::new(0.0, 0.0), max: Pos2::new(527.0, 454.0)};   

            let painter = ui.painter_at(dimensions);

            painter.debug_text([0.0,0.0].into(), Align2::LEFT_TOP, Color32::WHITE, format!("Last delay: {}", state.last_log_line_delay));

            let uv = Rect::from_min_max([0.0, 0.0].into(), [1.0, 1.0].into());

            let offset = 15.0;
            let ih = 56.0;
            let iw = 56.0;

            for (i, name) in state.volanic_hearts.iter().enumerate() {
                let bomb_number = (i+1).to_string();
                let i = i as f32;
                let ii = i + 1.0;
                let dimensions_heart = Rect{min: Pos2::new(0.0, i*ih + offset), max: Pos2::new(iw, (ii)*ih + offset)};

                painter.image(map.id(), dimensions_heart, uv, Color32::WHITE);
    
                painter.text([iw/2.0, (i*ih) + ih/2.0 + offset].into(), Align2::CENTER_CENTER, bomb_number, FontId::monospace(40.0), Color32::WHITE);
    
                let dimensions_heart_player_name = Rect{min: Pos2::new(iw, i*ih + offset), max: Pos2::new(256.0, (ii)*ih + offset)};
    
                painter.rect_filled(dimensions_heart_player_name, Rounding::none(), Color32::DARK_GRAY);
    
                painter.text([66.0, (i*ih) + ih/2.0 + offset].into(), Align2::LEFT_CENTER, name, FontId::monospace(25.0), Color32::WHITE);
            }
        });
    }
}


fn load_image_from_path(path: &str) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
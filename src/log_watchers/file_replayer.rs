use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    thread,
    time::Duration,
};

use chrono::NaiveDateTime;
use crossbeam::channel::Sender;

pub fn file_replayer(path: String, channel: Sender<String>) -> io::Result<()> {
    // get file
    println!("Started with {:?}", path);
    // get pos to end of file
    let f = File::open(&path)?;

    let line_reader = BufReader::new(&f);

    let mut time_of_last: Option<NaiveDateTime> = None;

    //let mut last_event = Instant::now();

    for line in line_reader.lines() {
        let line = line?;
        channel.send(line.clone()).unwrap();
        let mut split = line.split("  ");
        let date_time = split.next().unwrap().trim();
        let date_time = format!("2023/{date_time}");
        //let csv = split.next().unwrap();
        let date_time = NaiveDateTime::parse_from_str(&date_time, "%Y/%m/%d %X.%3f").unwrap();
        //handle_line(&state, csv);
        match time_of_last {
            Some(last_time) => {
                let time_chunk = date_time.signed_duration_since(last_time).to_std().unwrap();
                if time_chunk > Duration::from_millis(300) {
                    {
                        //let mut state = state.lock().unwrap();
                        //state.last_log_line_delay = last_event.elapsed().as_millis() as u32;
                        //last_event = Instant::now();
                        //state.ctx.request_repaint();
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

    //state.lock().unwrap().ctx.request_repaint();

    Ok(())
}

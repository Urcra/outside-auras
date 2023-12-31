use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    time::Instant,
};

use crossbeam::channel::{unbounded, Sender};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

pub fn file_watcher(path: String, channel: Sender<String>) -> notify::Result<()> {
    // get file
    println!("Started with {:?}", path);
    // get pos to end of file
    let mut f = File::open(&path)?;
    let mut pos = f.metadata()?.len();

    // set up watcher
    let (tx, rx) = unbounded();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    let mut last_event = Instant::now();

    // watch
    for res in rx {
        match res {
            Ok(_event) => {
                println!("ms since last event: {}", last_event.elapsed().as_millis());
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
                    channel.send(line).unwrap();
                    pos = tmp_pos;
                }
            }
            Err(error) => println!("{error:?}"),
        }
    }

    Ok(())
}

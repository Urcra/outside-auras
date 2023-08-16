use std::{sync::Arc, thread};

use crossbeam::channel::{unbounded, Receiver};
use ipc_channel::ipc::IpcSender;
use parking_lot::Mutex;

use super::{file_replayer, file_watcher};

pub struct LogDispatcher {
    _aura_channels: Arc<Mutex<Vec<IpcSender<String>>>>,
}

impl LogDispatcher {
    pub fn new(path: String, replay: bool, aura_channels: Vec<IpcSender<String>>) -> Self {
        let (tx, rx) = unbounded();
        let _aura_channels = Arc::new(Mutex::new(aura_channels));
        let aura_channels_clone = _aura_channels.clone();
        let log_receiver = rx;
        let log_sender = tx;
        thread::spawn(move || {
            if replay {
                file_replayer(path, log_sender).unwrap();
            } else {
                file_watcher(path, log_sender).unwrap();
            }
        });
        thread::spawn(move || Self::dispatch_logs(aura_channels_clone, log_receiver));

        Self { _aura_channels }
    }

    fn dispatch_logs(
        aura_channels: Arc<Mutex<Vec<IpcSender<String>>>>,
        log_receiver: Receiver<String>,
    ) {
        loop {
            match log_receiver.recv() {
                Ok(log_line) => {
                    for aura_channel in aura_channels.lock().iter() {
                        aura_channel.send(log_line.clone()).unwrap();
                    }
                }
                Err(e) => println!("Error dispatching log: {e}"),
            }
        }
    }
}

use rodio::Sink;

use std::fs::File;
use std::io::BufReader;

use std::sync::mpsc::{self, Sender};
use std::{thread, time};


pub struct Source<T> {
    source: Option<T>
}

pub struct AudioFileHandler {
    control_channel: Sender<bool>,
}

impl Source<File> {
    pub fn set_source(audio_source: File) -> Self {
        Self {
            source: Some(audio_source)
        }
    }

    pub fn get_source(&self) -> File {
        match &self.source {
            Some(s) => s.try_clone().unwrap(),
            None => panic!("Source not set!")
        }
    }
}


impl AudioFileHandler {
    pub fn initialize() -> Self {
        // TODO: This needs to be refactored.
        let (tx, rx) = mpsc::channel();
        Self {
            control_channel: tx,
        }
    }

    pub fn new(file: File) -> Self {
        let owned_file = file.try_clone().unwrap();
        let audio_source = rodio::Decoder::new(BufReader::new(owned_file)).unwrap();

        let (tx, rx) = mpsc::channel();
        let _t = thread::spawn(move || {
            let device = rodio::default_output_device().unwrap();
            let sink = Sink::new(&device);
            sink.pause();
            sink.append(audio_source);
            while let Ok(should_play) = rx.recv() {
                if should_play {
                    sink.play();
                } else {
                    sink.pause();
                }
            }
        });

        Self {
            control_channel: tx,
        }
    }

    pub fn play(&self) {
        self.control_channel.send(true).unwrap();
        thread::sleep(time::Duration::from_secs(180));
    }

    pub fn pause(&self) {
        self.control_channel.send(false).unwrap();
    }
}

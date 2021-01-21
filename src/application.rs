pub use crate::audio::AudioFileHandler;
pub use crate::input_handler::open_file;

extern crate gio;
extern crate gtk;

use cascade::cascade;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::FileChooserAction;
use std::env;
use std::sync::mpsc::{self, Sender};

use std::thread;

pub struct Application {
    control_channel: Sender<bool>,
    audio_handler: Option<AudioFileHandler>,
}

impl Application {
    pub fn show() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            ();
        }

        let uiapp = gtk::Application::new(
            Some("org.gtkrsnotes.demo"),
            gio::ApplicationFlags::FLAGS_NONE,
        )
        .expect("Application::new failed");

        uiapp.connect_activate(|app| {
            // Create the main window.
            let win = cascade! {
            gtk::ApplicationWindow::new(app);
            ..set_default_size(400, 350);
            ..set_title("Music Trainer");
            };

            // Setup the fixed panel grid to append additional widgets.
            let fixed = cascade! {
                gtk::Fixed::new();
            };

            win.add(&fixed);

            // TODO: Replace this eventually with a combobox with source selection.
            // Spotify, Apple/Google Music, whatever.
            // ATM this is just to load files.
            let file_chooser = cascade! {
                // Create FileChooser to select from filesystem.
                gtk::FileChooserButton::new("Select a file", FileChooserAction::Open);
                ..add_filter(&cascade!{
                    gtk::FileFilter::new();
                    ..add_pattern("*.mp3");
                    ..add_pattern("*.flac");
                    ..add_pattern("*.wav");
                    ..add_pattern("*.ogg");
                });
                ..set_size_request(325, 30);
                ..set_margin_start(30);
                ..set_margin_top(30);
                ..set_halign(gtk::Align::Center);
                ..set_valign(gtk::Align::Center);
                ..set_margin_end(65);
                ..connect_selection_changed(move |_f| {

                });
            };
            fixed.add(&file_chooser);

            let play_button = cascade! {
                gtk::Button::new();
                ..set_size_request(60, 35);
                ..set_margin_start(150);
                ..set_margin_top(85);
                ..set_halign(gtk::Align::Center);
                ..set_valign(gtk::Align::Center);
                ..connect_clicked(move |_btn| {
                    let f = match file_chooser.get_filename() {
                        Some(v) => v.into_os_string().into_string().unwrap(),
                        None => String::with_capacity(1),
                    };
                    AudioFileHandler::new(open_file(&f)).play();
                });
            };
            fixed.add(&play_button);

            win.show_all();
        });

        uiapp.run(&env::args().collect::<Vec<_>>());
    }

    //     pub fn handle_audio_request(&self, audio_file: String) -> Self{
    //         match self.audio_handler {
    //             Some(s) => {
    //                 self.audio_handler.unwrap().pause();
    //             },
    //             None => {
    //                 let mut new_handler = &AudioFileHandler::initialize();
    //                 let audio_file = open_file(&f);
    //                 new_handler = &AudioFileHandler::new(audio_file);
    //         }
    //     }
}

// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
use crate::gui::GuiMenu;
mod lib;

use eframe::egui;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};

const WINDOW_WIDTH: f32 = 380.0;
const WINDOW_HEIGHT: f32 = 560.0;

fn main() {
    // initialize the logger
    let _logger = Logger::try_with_str("info") // log info, warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(FileSpec::default().suppress_timestamp()) // no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Warn) // print warnings and errors also to the console
        .start()
        .unwrap();

    let mut options = eframe::NativeOptions::default();
    options.initial_window_size =
        Some(egui::Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    eframe::run_native(
        "MiniNotes",
        options,
        Box::new(|_cc| Box::new(GuiMenu::default())),
    );
}

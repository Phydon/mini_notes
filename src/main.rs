#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod gui;
use crate::gui::GuiMenu;

mod lib;

use eframe::egui;

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 960.0;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size =
        Some(egui::Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    eframe::run_native(
        "MiniNotes",
        options,
        Box::new(|_cc| Box::new(GuiMenu::default())),
    );
}

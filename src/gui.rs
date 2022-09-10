use crate::lib::*;

use eframe::egui;
use log::{warn, error};

use std::collections::BTreeMap;

const FILEPATH: &str = "./my_mininotes.txt";
const WINDOW_WIDTH: f32 = 380.0;
const WINDOW_HEIGHT: f32 = 560.0;
const CENTER: (f32, f32) = (
    (WINDOW_WIDTH - WINDOW_WIDTH * 0.60),
    (WINDOW_HEIGHT - WINDOW_HEIGHT * 0.5),
);
const PADDING: f32 = 10.0;

#[derive(Default, PartialEq)]
struct Note {
    note_txt: String,
    date: String,
}

#[derive(Default, PartialEq)]
pub struct GuiMenu {
    note: Note,
    storage: BTreeMap<String, String>,
    msg: String,
    warn: String,
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
}

impl eframe::App for GuiMenu {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // for the reset button
        let Self {
            note: _,
            storage: _,
            msg: _,
            warn: _,
            allowed_to_close: _,
            show_confirmation_dialog: _,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::LEFT),
                    |ui| {
                        ui.collapsing("Themes", |ui| {
                            egui::widgets::global_dark_light_mode_buttons(ui);
                        });
                    },
                );

                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::RIGHT),
                    |ui| {
                        // if ui.add(egui::Button::new("❌")).clicked() {
                        if ui.add(egui::Button::new("🇽")).clicked() {
                            self.on_close_event();
                        }
                        egui::reset_button(ui, self);
                    },
                );
            });
            ui.add_space(2.0);
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(3.0);
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("MiniNotes")
                        .size(50.0)
                        .strong()
                        .color(egui::Color32::from_rgb(73, 166, 153)),
                );
            });
            ui.add_space(3.0);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.vertical_centered_justified(|ui| {
                ui.hyperlink("leann.phydon@gmail.com");
                ui.label(egui::RichText::new("PoweredByRust").color(egui::Color32::from_rgb(156, 16, 39)));
            });
            ui.add_space(2.0);
        });

        // CentralPanel must be added after all other panels!
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(2.0);
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("✏ Take a Note: ")
                        .heading()
                        .strong()
                        .color(egui::Color32::from_rgb(6, 165, 149)),
                );
                ui.add(egui::TextEdit::multiline(&mut self.note.note_txt).hint_text("Enter your text here"));
            });
            ui.add_space(PADDING);

            ui.vertical_centered(|ui| {
                if ui
                    .add_sized([120., 25.], egui::Button::new("💾 Save"))
                    .clicked()
                {
                    self.note.date = get_date_and_time();
                    match store_notes(
                        &mut self.storage,
                        &self.note.date,
                        &self.note.note_txt,
                    ) {
                        Ok(()) => {
                            match write_to_file(FILEPATH, &self.storage) {
                                Ok(()) => {
                                    let success_msg: &str =
                                        "✔ Note written to file";
                                    self.msg = success_msg.to_string();
                                    self.warn.clear();
                                }
                                Err(err) => {
                                    let err_msg: &str =
                                        "✖ Unable to write to file";
                                    self.warn = err_msg.to_string();
                                    self.msg.clear();
                                    warn!("{err_msg}: {err}")
                                }
                            }
                        }
                        Err(err) => {
                            let err_msg: &str = "✖ Unable to store note";
                            self.warn = err_msg.to_string();
                            self.msg.clear();
                            warn!("{err_msg}: {err}")
                        }
                    }
                }

                ui.add_space(PADDING);
                if self.warn.is_empty() {
                    ui.label(
                        egui::RichText::new(format!("{}", self.msg))
                            .size(20.0)
                            .color(egui::Color32::from_rgb(78, 91, 173)),
                    );
                } else if self.msg.is_empty() {
                    ui.label(
                        egui::RichText::new(format!("{}", self.warn))
                            .size(20.0)
                            .color(egui::Color32::from_rgb(156, 16, 39)),
                    );
                } else {
                    error!("Unable to show messages: self.msg and self.warn are not empty");
                }
                ui.add_space(PADDING);
            });
            ui.separator();

            ui.add_space(2.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::RIGHT),
                    |ui| {
                        if ui
                            .add_sized(
                                [120., 25.],
                                egui::Button::new("⛃ Load notes"),
                            )
                            .clicked()
                        {
                            match read_file(FILEPATH) {
                                Ok(container) => {
                                    let info: &str = "✔ Notes loaded";
                                    self.msg = info.to_string();
                                    self.warn.clear();
                                    self.storage = container
                                }
                                _ => {
                                    let info: &str = "❓ No notes found";
                                    self.warn = info.to_string();
                                    self.msg.clear();
                                }
                            }
                        }
                    },
                );
            });

            ui.vertical_centered_justified(|ui| {
                egui::containers::ScrollArea::both().show(ui, |ui| {
                    for (key, value) in &self.storage {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                egui::RichText::new(format!("{}:: ", key,))
                                    .size(20.0)
                                    .color(egui::Color32::from_rgb(
                                        76, 116, 166,
                                    )),
                            );
                            ui.label(
                                egui::RichText::new(format!("{}", value,))
                                    .size(25.0)
                                    .color(egui::Color32::from_rgb(
                                        22, 146, 196,
                                    )),
                            );
                        });
                    }
                });
            });
            ui.add_space(2.0);

            if self.show_confirmation_dialog {
                // Show confirmation dialog:
                egui::Window::new("Do you want to quit?")
                    .collapsible(false)
                    .resizable(false)
                    .default_pos(CENTER)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Cancel").clicked() {
                                self.show_confirmation_dialog = false;
                            }

                            if ui.button("Yes!").clicked() {
                                self.allowed_to_close = true;
                                frame.close();
                            }
                        });
                    });
            }
        });
    }
}

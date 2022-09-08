use crate::lib::*;

use eframe::egui;

use std::collections::BTreeMap;

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 960.0;
const CENTER: (f32, f32) = (
    (WINDOW_WIDTH - WINDOW_WIDTH * 0.60),
    (WINDOW_HEIGHT - WINDOW_HEIGHT * 0.5),
);
const PADDING: f32 = 20.0;

#[derive(Default, PartialEq)]
pub struct GuiMenu {
    note_txt: String,
    idx: String,
    storage: BTreeMap<String, String>,
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
            note_txt: _,
            idx: _,
            storage: _,
            allowed_to_close: _,
            show_confirmation_dialog: _,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.collapsing("Themes", |ui| {
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    egui::reset_button(ui, self);
                });
            
            });
            ui.add_space(2.0);
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("MiniNotes").size(50.0).strong().color(egui::Color32::from_rgb(171, 39, 79)));
            });
            ui.add_space(8.0);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            ui.vertical_centered(|ui| {
                let tooltip_text = "PoweredByRust";
                ui.hyperlink("leann.phydon@gmail.com")
                    .on_hover_text(tooltip_text);
            });
            ui.add_space(2.0);
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                for i in 1..=60 {
                    ui.label(format!("just some txt {}", i as usize));
                }
            });
            ui.add_space(2.0);
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Right");
            });
            ui.add_space(2.0);
        });

        // CentralPanel must be added after all other panels!
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(2.0);
            ui.heading("Take a Note");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Enter your note here: ");
                ui.text_edit_singleline(&mut self.note_txt);
            });
            ui.add_space(PADDING);
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::from_rgb(128, 140, 255), "Your Note: "); // Shortcut version
                ui.label(egui::RichText::new(format!(
                    "{}",
                    self.note_txt,
                )).size(20.0).italics());
            });

            ui.vertical_centered(|ui| {
                if ui.add_sized([240., 80.], egui::Button::new("Save")).on_hover_text("save your note").clicked() {
                    self.idx = get_date_and_time();
                    match store_notes(&mut self.storage, &self.idx, &self.note_txt) {
                        Ok(()) => {
                            println!("Note stored");
                        }                        
                        Err(err) => println!("Unable to store note: {err}"),
                    }
                }
                ui.add_space(PADDING + 10.0);
            });
            ui.separator();
            ui.separator();

            ui.add_space(2.0);
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                for (key, value) in &self.storage{
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!(
                            "{}:: ",
                            key,
                        )).size(20.0).color(egui::Color32::from_rgb(110, 255, 110)));
                        ui.label(egui::RichText::new(format!(
                            "{}",
                            value,
                        )).size(25.0));
                    });
                }
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

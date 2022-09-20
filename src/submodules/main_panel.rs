use eframe::{egui::{CentralPanel, self, menu, Layout}, emath::Align};

use crate::Flasher;

impl Flasher {
    pub fn render_main_windows(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui|{

            menu::bar(ui, |ui|{
                egui::ComboBox::from_label("Select Your Soldering Iron.")
                    .selected_text(format!("{}", self.config.iron))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.iron, "Pinecil V1".to_string(), "Pinecil V1");
                        ui.selectable_value(&mut self.config.iron, "Pinecil V2".to_string(), "Pinecil V2 Work in Progress");
                        ui.selectable_value(&mut self.config.iron, "TS100".to_string(), "Ts100 Work in Progress");
                        ui.selectable_value(&mut self.config.iron, "TS80".to_string(), "Ts80 Work in Progress");
                    }
                );
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                    egui::ComboBox::from_label("Specify Release Version")
                        .selected_text(format!("{}", self.config.version))
                        .show_ui(ui, |ui| {
                            if self.config.versions_checked {
                                for i in &self.config.vers {
                                    ui.selectable_value(&mut self.config.version, i.clone(), i);
                                }
                            }

                        }
                    );
                });
            });
            
            menu::bar(ui, |ui|{
                 egui::ComboBox::from_label("Select Your Language.")
                    .selected_text(format!("{}", self.config.lang))
                    .show_ui(ui, |ui| {
                        for i in &self.config.langs {
                            ui.selectable_value(&mut self.config.lang, i.clone(), i);
                        }
                    }
                );               
            });

            ui.vertical_centered(|ui|{
                if ui.button("Flash!").clicked() {
                    Flasher::download(self);
                    Flasher::flash(self);
                };
            })
        });
    }
}
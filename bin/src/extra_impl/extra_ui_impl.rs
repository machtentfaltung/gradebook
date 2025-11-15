// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;

pub trait ExtraUiImpl {
    fn theme_combo_box(&mut self);
    fn custom_heading(&mut self, text: impl ToString);
}

impl ExtraUiImpl for egui::Ui {
    fn theme_combo_box(&mut self) {
        let mut ui_theme_preference = self.ctx().options(|opt| opt.theme_preference);
        self.vertical(|ui| {
            ui.label("Theme");
            egui::ComboBox::from_id_salt("theme_combo_box")
                .selected_text(match ui_theme_preference {
                    egui::ThemePreference::System => "System",
                    egui::ThemePreference::Dark => "Dark",
                    egui::ThemePreference::Light => "Light",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut ui_theme_preference,
                        egui::ThemePreference::System,
                        "System",
                    );
                    ui.selectable_value(
                        &mut ui_theme_preference,
                        egui::ThemePreference::Dark,
                        "Dark",
                    );
                    ui.selectable_value(
                        &mut ui_theme_preference,
                        egui::ThemePreference::Light,
                        "Light",
                    );
                });
        });
        self.ctx().set_theme(ui_theme_preference);
    }

    fn custom_heading(&mut self, text: impl ToString) {
        self.add_space(3.0);

        let text = egui::RichText::new(text.to_string())
            .font(egui::FontId {
                size: 24.0,
                family: egui::FontFamily::Name("Autobahn".into()),
            })
            .strong();

        self.heading(text);
    }
}


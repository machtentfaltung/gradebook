// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use eframe::egui;

use crate::extra_impl::extra_ctx_impl::ExtraCtxImpl;
use crate::extra_impl::extra_ui_impl::ExtraUiImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    compact: bool,
}

impl Default for Application {
    fn default() -> Self {
        Self { compact: false }
    }
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "DINish".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/DINish-Regular.ttf")).into(),
        );

        fonts.font_data.insert(
            "Autobahn".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/Autobahn.ttf")).into(),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "DINish".to_owned());

        let mut new_family = BTreeMap::new();

        new_family.insert(
            egui::FontFamily::Name("Autobahn".into()),
            vec!["Autobahn".to_owned()],
        );

        fonts.families.append(&mut new_family);

        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.compact = ctx.is_compact();

        egui::Window::new("Gradebook").show(ctx, |ui| {
            ui.custom_heading("Gradebook");
            ui.theme_combo_box();
            ui.heading(format!("Compact: {}", self.compact));
        });
    }
}


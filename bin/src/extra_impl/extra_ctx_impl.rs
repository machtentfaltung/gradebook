// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;

pub const COMPACT_THRESHOLD: f32 = 450.;

pub trait ExtraCtxImpl {
    fn is_compact(&self) -> bool;
}

impl ExtraCtxImpl for egui::Context {
    fn is_compact(&self) -> bool {
        let screen_size = self.input(|i| i.content_rect());
        screen_size.width() <= COMPACT_THRESHOLD
    }
}


#![allow(unused_must_use)]

use eframe::egui;

struct Minesweeper {
    mines: i32,
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("test");
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| Ok(Box::new(Minesweeper { mines: 24 }))),
    );
}

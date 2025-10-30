#![allow(unused_must_use)]

use eframe::egui;

struct Minesweeper {
    mines: i32,
    height: i32,
    width: i32,
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("config").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add(egui::Slider::new(&mut self.mines, 0..=self.height*self.width -2).text("Mines"));
                ui.add(egui::Slider::new(&mut self.height, 0..=200).text("Height"));
                ui.add(egui::Slider::new(&mut self.width, 0..=200).text("Width"));
                if ui.button("Start Game").clicked() {
                    // generate the map
                }
            });
        });
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
        Box::new(|_cc| {
            Ok(Box::new(Minesweeper {
                mines: 24,
                height: 24,
                width: 24,
            }))
        }),
    );
}

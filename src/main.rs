#![allow(unused_must_use)]

use eframe::egui;

struct Minesweeper {
    mines: i32,
    height: i32,
    width: i32,
    generated: bool,
    board: Vec<u8>,
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.generated {
            // if not generated generate easy
            self.board.reserve((self.height * self.width) as usize); //resers so there is no reasaining
            for _ in 0..self.height {
                for _ in 0..self.width {
                    self.board.push(0);
                }
            }
            self.generated = true;
        }
        egui::SidePanel::left("config").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.mines, 1..=self.height * self.width - 2)
                        .text("Mines"),
                );
                ui.add(egui::Slider::new(&mut self.height, 3..=50).text("Height"));
                ui.add(egui::Slider::new(&mut self.width, 3..=72).text("Width"));
                if ui.button("Regenerate game").clicked() {
                    self.generated = false;
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            for _ in 0..self.height {
                ui.columns(self.width as usize, |cols| {
                    for i in 0..self.width {
                        cols[i as usize]
                            .vertical_centered_justified(|ui| if ui.button("B").clicked() {});
                    }
                });
            }
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
                generated: false,
                board: Vec::new(),
            }))
        }),
    );
}

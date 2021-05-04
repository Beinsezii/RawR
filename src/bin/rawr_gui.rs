use copypasta_ext::prelude::*;
use eframe::{egui, epi};

struct App {
    input: String,
    output: String,
    uwu: bool,
    mock: bool,
    mock_min: u32,
    mock_max: u32,
    cipher: String,
    cipher_decode: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            uwu: true,
            mock: false,
            mock_min: 1,
            mock_max: 4,
            cipher: "".to_owned(),
            cipher_decode: false,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "egui name"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        // you can go rawr!(...) to update the output on .changed() == true
        macro_rules! rawr {
            () => {
                rawr::rawr(
                    self.uwu,
                    self.mock,
                    &self.cipher,
                    rawr::Source::String(&mut self.input),
                    rawr::Source::String(&mut self.output),
                    rawr::Args {
                        mock_min: self.mock_min,
                        mock_max: self.mock_max,
                        cipher_decode: self.cipher_decode,
                        ..Default::default()
                    },
                )
            };
            ($x:expr) => {
                if $x {
                    rawr!()
                }
            };
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(3, |cols| {
                if cols[0].button("Paste").clicked() {
                    self.input = copypasta_ext::x11_bin::ClipboardContext::new()
                        .expect("Clip provider fail")
                        .get_contents()
                        .expect("Clip get fail");
                    rawr!();
                }
                rawr!(cols[0]
                    .add(
                        egui::widgets::TextEdit::multiline(&mut self.input)
                            .hint_text("Write text here to process")
                    )
                    .changed());

                rawr!(cols[1].checkbox(&mut self.uwu, "UwU").changed());

                cols[1].horizontal(|col| {
                    rawr!(col.checkbox(&mut self.mock, "mOcK").changed());
                    col.spacing_mut().item_spacing = egui::vec2(2.0, 0.0);
                    rawr!(col
                        .add(
                            egui::widgets::DragValue::new(&mut self.mock_min)
                                .clamp_range(1..=u32::MAX),
                        )
                        .changed());
                    col.heading(":");
                    rawr!(col
                        .add(
                            egui::widgets::DragValue::new(&mut self.mock_max)
                                .clamp_range(1..=u32::MAX),
                        )
                        .changed());
                });

                cols[1].horizontal(|col| {
                    col.label("Cipher:");
                    rawr!(col
                        .text_edit_singleline(&mut self.cipher)
                        .on_hover_text("Password for cipher")
                        .changed());
                });
                rawr!(cols[1]
                    .checkbox(&mut self.cipher_decode, "Cipher Decode")
                    .on_hover_text("Decode an existing cipher instead of creating a new one")
                    .changed());

                if cols[2].button("Copy").clicked() {
                    copypasta_ext::x11_bin::ClipboardContext::new()
                        .expect("Clip provider fail")
                        .set_contents(self.output.clone())
                        .expect("Clip set fail");
                    rawr!();
                }
                cols[2].add(
                    egui::widgets::TextEdit::multiline(&mut self.output)
                        .hint_text("Processed text will appear here"),
                );
            });
        });
    }
}

fn main() {
    eframe::run_native(Box::new(App::default()));
}

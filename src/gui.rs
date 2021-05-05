use copypasta_ext::prelude::*;
use eframe::{egui, epi};

pub struct App {
    pub input: String,
    pub output: String,
    pub update: bool,
    pub uwu: bool,
    pub mock: bool,
    pub mock_min: usize,
    pub mock_max: usize,
    pub cipher: String,
    pub cipher_decode: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            update: true,
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
        macro_rules! up {
            ($x:expr) => {
                if $x {
                    self.update = true
                }
            };
        }

        if self.update {
            rawr::rawr(rawr::Rawrgs {
                uwu: self.uwu,
                mock: self.mock,
                cipher: self.cipher.clone(),
                input: rawr::Source::String(&mut self.input),
                output: rawr::Source::String(&mut self.output),
                mock_range: self.mock_min..=self.mock_max,
                cipher_decode: self.cipher_decode,
                ..Default::default()
            });
            self.update = false;
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(3, |cols| {
                if cols[0].button("Paste").clicked() {
                    self.input = copypasta_ext::x11_bin::ClipboardContext::new()
                        .expect("Clip provider fail")
                        .get_contents()
                        .expect("Clip get fail");
                    self.update = true;
                }
                up!(cols[0]
                    .add(
                        egui::widgets::TextEdit::multiline(&mut self.input)
                            .hint_text("Write text here to process")
                    )
                    .changed());

                up!(cols[1].checkbox(&mut self.uwu, "UwU").changed());

                cols[1].horizontal(|col| {
                    up!(col.checkbox(&mut self.mock, "mOcK").changed());
                    col.spacing_mut().item_spacing = egui::vec2(2.0, 0.0);
                    if col
                        .add(
                            egui::widgets::DragValue::new(&mut self.mock_min)
                                .clamp_range(1..=usize::MAX),
                        )
                        .changed()
                    {
                        // updating the clamp doesn't push the new val
                        if self.mock_min > self.mock_max {
                            self.mock_max = self.mock_min
                        };
                        self.update = true;
                    };
                    col.heading(":");
                    up!(col
                        .add(
                            egui::widgets::DragValue::new(&mut self.mock_max)
                                .clamp_range(self.mock_min..=usize::MAX),
                        )
                        .changed());
                });

                cols[1].horizontal(|col| {
                    col.label("Cipher:");
                    up!(col
                        .text_edit_singleline(&mut self.cipher)
                        .on_hover_text("Password for cipher")
                        .changed());
                });
                up!(cols[1]
                    .checkbox(&mut self.cipher_decode, "Cipher Decode")
                    .on_hover_text("Decode an existing cipher instead of creating a new one")
                    .changed());

                if cols[2].button("Copy").clicked() {
                    copypasta_ext::x11_bin::ClipboardContext::new()
                        .expect("Clip provider fail")
                        .set_contents(self.output.clone())
                        .expect("Clip set fail");
                    self.update = true;
                }
                cols[2].add(
                    egui::widgets::TextEdit::multiline(&mut self.output)
                        .hint_text("Processed text will appear here"),
                );
            });
        });
    }
}

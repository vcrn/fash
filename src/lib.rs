mod hasher;

use eframe::egui;
use hasher::compute_hash;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;
use std::io::Write;

pub fn run() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "File hash verifier",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(Fash::default())
        }),
    );
}

/// Used by radio buttons to select hashing algorithm
#[derive(PartialEq)]
pub enum RadioHash {
    Sha256,
    Sha1,
    Md5,
}

impl Default for RadioHash {
    fn default() -> Self {
        RadioHash::Sha256
    }
}

#[derive(Default)]
struct Fash {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    entered_hash: String,
    file_path: String,
    computed_hash: String,
    radio_hash: RadioHash,
    output_file_path: String,
}

impl eframe::App for Fash {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    egui::global_dark_light_mode_buttons(ui);
                    ui.label("Drag-and-drop a file onto the window!");
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                        }
                        if let Some(picked_path) = &self.picked_path {
                            self.file_path = picked_path.clone();
                        }
                        self.picked_path = None;
                        self.computed_hash = "".to_string();
                    }
                });

                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.entered_hash)
                            .hint_text("Enter hash to compare file hash, to or leave blank"),
                    );
                    if !self.entered_hash.is_empty() && ui.button("Remove whitespace").clicked() {
                        self.entered_hash.retain(|c| !c.is_whitespace());
                    };
                });
            });

            if !self.file_path.is_empty() {
                ui.group(|ui| {
                    ui.label("Selected file:");
                    ui.label(&self.file_path);

                    if ui.button("Compute file hash").clicked() {
                        let hash_result = match self.radio_hash {
                            RadioHash::Sha256 => compute_hash::<Sha256>(&self.file_path),
                            RadioHash::Sha1 => compute_hash::<Sha1>(&self.file_path),
                            RadioHash::Md5 => compute_hash::<Md5>(&self.file_path),
                        };
                        self.computed_hash = match hash_result {
                            Ok(hash) => {
                                self.output_file_path =
                                    generate_output_file_path(&self.file_path, &self.radio_hash);
                                hash
                            }
                            _ => {
                                self.output_file_path = "".to_string();
                                "ERROR: Couldn't read file".to_string()
                            }
                        };
                    }
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.radio_hash, RadioHash::Sha256, "SHA256");
                        ui.radio_value(&mut self.radio_hash, RadioHash::Sha1, "SHA1");
                        ui.radio_value(&mut self.radio_hash, RadioHash::Md5, "MD5");
                    });
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                for file in &self.dropped_files {
                    let mut file_path = if let Some(path) = &file.path {
                        path.display().to_string()
                    } else if !file.name.is_empty() {
                        file.name.clone()
                    } else {
                        "???".to_owned()
                    };
                    if let Some(bytes) = &file.bytes {
                        file_path += &format!(" ({} bytes)", bytes.len());
                    }
                    self.file_path = file_path;
                }
                self.dropped_files.clear();
                self.computed_hash = "".to_string();
            }

            // Display hash and info
            ui.label("\n");

            if !self.computed_hash.is_empty() {
                ui.group(|ui| {
                    ui.label("Hash is");
                    selectable_text(ui, self.computed_hash.as_str());

                    if !self.entered_hash.is_empty() {
                        ui.label(format!("Hashes match: {}", self.computed_hash == self.entered_hash));
                    }
                });
                if !self.output_file_path.is_empty() {
                    if ui.button("Save hash to file").clicked() {
                        if let Err(e) = write_file(&self.computed_hash, &self.file_path) {
                            let err_mess = "Unable to save hash to file!";
                            println!("{}! Error: {}", err_mess, e);
                            self.output_file_path = err_mess.to_string();
                        };
                    }
                    ui.label(format!("Output file path: {}", self.output_file_path));
                    ui.label("NOTE: IF A FILE MATCHING THE OUTPUT FILE PATH ALREADY EXISTS, SAVING THE HASH WILL OVERWRITE IT");
                }
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}

/// Returns a selectable un-editable text box
fn selectable_text(ui: &mut egui::Ui, mut text: &str) {
    ui.add(egui::TextEdit::multiline(&mut text));
}

/// Preview hovering files
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                text += &format!("\n{}", path.display());
            } else if !file.mime.is_empty() {
                text += &format!("\n{}", file.mime);
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

/// Generates output_file_path by removing file extension and appending "_[hash name].txt"
/// # Example
///
/// ```
/// let input_file_path = "folder1/folder2/some_pic.jpg";
/// let hash_algorithm = &fash::RadioHash::Sha256;
/// let expected_output_file_path = "folder1/folder2/some_pic_sha256.txt";
/// let generated_output_file_path = fash::generate_output_file_path(input_file_path, hash_algorithm);
/// assert_eq!(generated_output_file_path, expected_output_file_path);
/// ```
pub fn generate_output_file_path(input_file_path: &str, hash_algorithm: &RadioHash) -> String {
    let hash_append = match hash_algorithm {
        RadioHash::Sha256 => "_sha256",
        RadioHash::Sha1 => "_sha1",
        RadioHash::Md5 => "_md5",
    };

    let mut output_file_path = input_file_path.split('.').next().unwrap().to_string(); // Returns first element of iterator. TODO: remove unwrap()
    output_file_path.push_str(hash_append);
    output_file_path.push_str(".txt");

    output_file_path
}

/// Writes data to output_file_path
fn write_file(data: &str, output_file_path: &str) -> Result<(), std::io::Error> {
    let mut f = std::fs::File::create(output_file_path)?;
    f.write_all(data.as_bytes())?;

    Ok(())
}

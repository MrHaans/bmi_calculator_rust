use eframe::egui::{self, CentralPanel, TextEdit, Button, Color32, Frame, RichText, FontId};

#[derive(Default)]
struct BMIApp {
    height_input: String,
    weight_input: String,
    bmi_result: String,
    status: String,
}

impl eframe::App for BMIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set the background to white using Frame
        CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE)) // Set white background
            .show(ctx, |ui| {
                // CENTER THE LAYOUT
                ui.vertical_centered(|ui| {
                    ui.group(|ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            // Set the BMI Calculator text with larger size, bold, and black
                            let heading = RichText::new("BMI Calculator")
                                .font(FontId::proportional(20.0 * 1.10)) // Increase font size by 10%
                                .color(Color32::BLACK); // Set text color to black
                            ui.label(heading);

                            // Set text color to black for labels
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Enter your height (in cm): ")
                                        .color(Color32::BLACK)
                                );
                                ui.add(TextEdit::singleline(&mut self.height_input).hint_text("Height"));
                            });

                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Enter your weight (in kg): ")
                                        .color(Color32::BLACK)
                                );
                                ui.add(TextEdit::singleline(&mut self.weight_input).hint_text("Weight"));
                            });

                            if ui.add(Button::new("Calculate BMI")).clicked() {
                                let height_cm: f32 = self.height_input.parse().unwrap_or(0.0);
                                let weight: f32 = self.weight_input.parse().unwrap_or(0.0);

                                let height = height_cm / 100.0;

                                if height > 0.0 && weight > 0.0 {
                                    let bmi = weight / (height * height);
                                    self.bmi_result = format!("{:.2}", bmi);

                                    self.status = if bmi < 18.5 {
                                        "Underweight".to_string()
                                    } else if bmi < 24.9 {
                                        "Normal weight".to_string()
                                    } else if bmi < 29.9 {
                                        "Overweight".to_string()
                                    } else {
                                        "Obesity".to_string()
                                    };
                                } else {
                                    self.bmi_result = "Invalid input".to_string();
                                    self.status = "".to_string();
                                }
                            }

                            ui.separator();

                            // Set text color to black for results
                            ui.label(
                                RichText::new(format!("Your BMI: {}", self.bmi_result))
                                    .color(Color32::BLACK)
                            );
                            ui.label(
                                RichText::new(format!("Status: {}", self.status))
                                    .color(Color32::BLACK)
                            );
                        });
                    });
                });
            });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    let _ = eframe::run_native(
        "BMI Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(BMIApp::default()))),
    );
}

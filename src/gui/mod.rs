use egui::{Color32, Pos2, Rect};

use crate::{
    Config,
    rad::{
        BASELINE, MIN_MAGNITUDE, RadiationInfo, RadioactiveMaterial, decay_time,
        get_severity_color, grid_2d,
    },
};

#[derive(Default)]
pub struct RadiationApp {
    config: Config,

    /// Magnitude is the center of the radiation source, in Sieverts.
    rad_source: RadiationInfo,
    rad_magnitude_text: String,
    rad_result: Option<Vec<Vec<RadiationInfo>>>,

    rad_material_text: String,
    rad_material_mb: f64,

    selected_material: RadioactiveMaterial,

    cell_size: f32,
}

const ROWS: usize = 100;
const COLUMNS: usize = 100;

impl RadiationApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            cell_size: 1.0,
            ..Default::default()
        }
    }

    fn regenerate(&mut self) {
        self.rad_result = Some(grid_2d(&self.rad_source, ROWS, COLUMNS, self.cell_size));
    }
}

impl eframe::App for RadiationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let height = egui::TopBottomPanel::top("settings")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let last_sv_src = self.rad_magnitude_text.clone();

                    ui.label("Radiation Source (Sv): ");
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.rad_magnitude_text)
                                .desired_width(50.),
                        )
                        .lost_focus()
                        && self.rad_magnitude_text != last_sv_src
                        && let Ok(m) = self.rad_magnitude_text.parse()
                    {
                        self.rad_source.magnitude = m;
                        self.regenerate();
                    };

                    ui.label("Cell Size");
                    if ui
                        .add(egui::Slider::new(&mut self.cell_size, 1.0..=128.0))
                        .changed()
                    {
                        self.regenerate()
                    }

                    ui.label("Radioactive Material (mB): ");
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.rad_material_text)
                                .desired_width(50.),
                        )
                        .lost_focus()
                        && let Ok(m) = self.rad_material_text.parse()
                        && self.rad_material_mb != m
                    {
                        self.rad_material_mb = m;
                        self.rad_source.magnitude =
                            self.selected_material.value() * self.rad_material_mb;
                        self.regenerate()
                    };

                    let before = self.selected_material;
                    egui::ComboBox::from_label("Radioactive Material")
                        .selected_text(self.selected_material.name())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.selected_material,
                                RadioactiveMaterial::None,
                                "None",
                            );
                            ui.selectable_value(
                                &mut self.selected_material,
                                RadioactiveMaterial::NuclearWaste,
                                "Nuclear Waste",
                            );
                            ui.selectable_value(
                                &mut self.selected_material,
                                RadioactiveMaterial::Plutonium,
                                "Plutonium",
                            );
                            ui.selectable_value(
                                &mut self.selected_material,
                                RadioactiveMaterial::Polonium,
                                "Polonium",
                            );
                        });

                    if self.selected_material != before {
                        self.rad_source.magnitude =
                            self.selected_material.value() * self.rad_material_mb;
                        self.regenerate();
                    }
                });
                ui.add_space(5.0);
            })
            .response
            .rect
            .height();

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(rad_grid) = &self.rad_result {
                let size = ui.available_size();

                let (tile_size, offset) = if size.x > size.y {
                    (size.y / COLUMNS as f32, (size.x - size.y) / 2.)
                } else {
                    (size.x / COLUMNS as f32, (size.y - size.x) / 2.)
                };

                for (x, row) in rad_grid.iter().enumerate().take(ROWS + 1) {
                    for (y, rad_info) in row.iter().enumerate().take(COLUMNS + 1) {
                        let (xa, ya) = if size.x > size.y {
                            (
                                x as f32 * tile_size + offset,
                                y as f32 * tile_size + height + 2.0,
                            )
                        } else {
                            (
                                x as f32 * tile_size,
                                y as f32 * tile_size + offset + height + 2.0,
                            )
                        };

                        let rect = Rect::from_two_pos(
                            Pos2::new(xa, ya),
                            Pos2::new(xa + tile_size, ya + tile_size),
                        );
                        ui.painter()
                            .rect_filled(rect, 0.0, get_severity_color(rad_info.magnitude));
                        ui.painter().rect_stroke(
                            rect,
                            0.0,
                            egui::Stroke::new(0.5, Color32::BLACK),
                            egui::StrokeKind::Inside,
                        );
                        let response =
                            ui.interact(rect, format!("x{}y{}", x, y).into(), egui::Sense::click());

                        egui::Popup::menu(&response)
                            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                            .show(|ui| {
                                ui.set_min_width(30.0);
                                ui.label(rad_info.pos.to_string());
                                if rad_info.magnitude > BASELINE {
                                    ui.label(
                                        crate::unit::MeasurementUnit::unit(rad_info.magnitude)
                                            .display(rad_info.magnitude, "Sv".to_string(), 3)
                                            .to_string(),
                                    );
                                    if rad_info.magnitude > MIN_MAGNITUDE {
                                        let ticks =
                                            decay_time(&self.config, rad_info.magnitude, false);
                                        ui.label(format!(
                                            "Time to decay to 10 uSv/h: {} ({} ticks)",
                                            dur_to_hms(&std::time::Duration::from_secs(ticks / 20)),
                                            ticks
                                        ));
                                    }
                                } else {
                                    ui.label(format!(
                                        "Background Radiation ({})",
                                        crate::unit::MeasurementUnit::unit(BASELINE).display(
                                            BASELINE,
                                            "Sv".to_string(),
                                            3
                                        )
                                    ));
                                }
                            });
                    }
                }
            }
        });
    }
}

pub fn dur_to_hms(duration: &std::time::Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;
    format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
}

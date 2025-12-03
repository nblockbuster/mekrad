use egui::Color32;
use glam::Vec3;
use tracing::instrument;

use crate::Config;

pub const BASELINE: f64 = 0.000_000_100; // 100 nSv/h
pub const MIN_MAGNITUDE: f64 = 0.000_010; // 10 uSv/h

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioactiveMaterial {
    #[default]
    None,
    // Spent is the same as basic waste
    NuclearWaste,
    Plutonium,
    Polonium,
}

impl RadioactiveMaterial {
    pub fn value(&self) -> f64 {
        // All values are in Sv/mB
        match self {
            Self::None => 0.0,
            Self::NuclearWaste => 0.010,
            Self::Plutonium => 0.020,
            Self::Polonium => 0.050,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::NuclearWaste => "Nuclear Waste",
            Self::Plutonium => "Plutonium",
            Self::Polonium => "Polonium",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RadiationInfo {
    pub pos: Vec3,
    pub magnitude: f64,
}

/// Appx. how long in ticks radiation of magnitude `mag` will take to decay.
#[instrument]
pub fn decay_time(cfg: &Config, mag: f64, is_source: bool) -> u64 {
    let decay_rate = if is_source {
        cfg.source_decay_rate
    } else {
        cfg.target_decay_rate
    };

    let mut local_mag = mag;
    let mut ticks = 0;

    while local_mag > MIN_MAGNITUDE {
        local_mag *= decay_rate;
        ticks += 20;
    }

    ticks
}

/// Amount of Sieverts/h the destination position `dst` recieves from the RadiationInfo `src`
pub fn compute_exposure_magnitude(src: &RadiationInfo, dst: Vec3) -> f64 {
    src.magnitude / 1.0_f64.max(dst.distance_squared(src.pos) as f64)
}

pub fn get_severity_color(magnitude: f64) -> Color32 {
    match magnitude {
        f64::MIN..=BASELINE => Color32::LIGHT_GREEN,
        // 10 uSv/h
        BASELINE..0.00001 => Color32::GRAY,
        // 1 mSv/h
        0.00001..0.001 => Color32::YELLOW,
        // 100 mSv/h
        0.001..0.1 => Color32::ORANGE,
        // 100 Sv/h
        0.1..10.0 => Color32::RED,
        _ => Color32::DARK_RED,
    }
}

/// Generates a 2D grid of radiation results for a given amount of rows and columns, from a RadiationInfo `src`
#[instrument]
pub fn grid_2d(
    src: &RadiationInfo,
    rows: usize,
    columns: usize,
    cell_size: f32,
) -> Vec<Vec<RadiationInfo>> {
    let mut info = Vec::new();

    let p = src.pos;
    let origin = Vec3::new(
        p.x - columns as f32 * cell_size,
        p.y,
        p.z - rows as f32 * cell_size,
    );
    let center = p.midpoint(origin);

    for x in 0..=rows {
        let mut col = Vec::new();
        for y in 0..=columns {
            let pos = Vec3::new(
                // We're flipping X and Y here so that -Z is up/north and -X is left/west
                center.z + x as f32 * cell_size,
                origin.y,
                center.x + y as f32 * cell_size,
            );
            col.push(RadiationInfo {
                pos,
                magnitude: compute_exposure_magnitude(src, pos),
            })
        }
        info.push(col)
    }

    info
}

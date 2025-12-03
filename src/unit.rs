use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum MeasurementUnit {
    Femto,
    Pico,
    Nano,
    Micro,
    Milli,
    Base,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
}

impl MeasurementUnit {
    pub fn unit(val: f64) -> Self {
        let mut f = MeasurementUnit::Base;

        let units = Self::iter().collect::<Vec<_>>();
        for (i, unit) in units.clone().into_iter().enumerate() {
            if (i == 0 && val < unit.value())
                || i + 1 >= units.len()
                || (val >= unit.value() && val < units[i + 1].value())
            {
                f = unit.clone();
                break;
            }
        }
        f
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            MeasurementUnit::Femto => "f",
            MeasurementUnit::Pico => "p",
            MeasurementUnit::Nano => "n",
            MeasurementUnit::Micro => "µ",
            MeasurementUnit::Milli => "m",
            MeasurementUnit::Base => "",
            MeasurementUnit::Kilo => "k",
            MeasurementUnit::Mega => "M",
            MeasurementUnit::Giga => "G",
            MeasurementUnit::Tera => "T",
            MeasurementUnit::Peta => "P",
            MeasurementUnit::Exa => "E",
            MeasurementUnit::Zetta => "Z",
            MeasurementUnit::Yotta => "Y",
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            MeasurementUnit::Femto => 0.000_000_000_000_001,
            MeasurementUnit::Pico => 0.000_000_000_001,
            MeasurementUnit::Nano => 0.000_000_001,
            MeasurementUnit::Micro => 0.000_001,
            MeasurementUnit::Milli => 0.001,
            MeasurementUnit::Base => 1.,
            MeasurementUnit::Kilo => 1_000.0,
            MeasurementUnit::Mega => 1_000_000.0,
            MeasurementUnit::Giga => 1_000_000_000.0,
            MeasurementUnit::Tera => 1_000_000_000_000.0,
            MeasurementUnit::Peta => 1_000_000_000_000_000.0,
            MeasurementUnit::Exa => 1_000_000_000_000_000_000.0,
            MeasurementUnit::Zetta => 1_000_000_000_000_000_000_000.0,
            MeasurementUnit::Yotta => 1_000_000_000_000_000_000_000_000.0,
        }
    }

    pub fn process(&self, val: f64) -> f64 {
        val / self.value()
    }

    pub fn display(&self, val: f64, unit: String, decimal_places: usize) -> String {
        format!(
            "{v:.prec$} {0}{1}",
            self.symbol(),
            unit,
            v = if val.is_sign_negative() {
                self.process(-val)
            } else {
                self.process(val)
            },
            prec = decimal_places,
        )
    }
}

use lm_sensors::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use log;

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    pub chip: String,
    pub temperature: f64,
}

fn list_sensors() -> Result<Vec<Temperature>, Box<dyn std::error::Error>> {
    let mut values = Vec::new();
    let sensors = lm_sensors::Initializer::default().initialize()?;
    let re = Regex::new(r"temp\d_input").unwrap();
    // Print all chips.
    for chip in sensors.chip_iter(None) {
        if let Some(path) = chip.path() {
            log::trace!("chip: {} at {} ({})", chip, chip.bus(), path.display());
        } else {
            log::trace!("chip: {} at {}", chip, chip.bus());
        }

        // Print all features of the current chip.
        for feature in chip.feature_iter() {
            let name = feature.name().transpose()?.unwrap_or("N/A");
            log::trace!("    {}: {}", name, feature);

            // Print all sub-features of the current chip feature.
            for sub_feature in feature.sub_feature_iter() {
                if let Ok(value) = sub_feature.value() {
                    if re.is_match(&sub_feature.to_string().as_str()) {
                        values.push(Temperature{chip: name.to_string().to_owned(), temperature: value.raw_value()});
                    }

                    log::trace!("        {}: {}", sub_feature, value);
                } else {
                    log::trace!("        {}: N/A", sub_feature);
                }
            }
        }
    }

    Ok(values)
}

pub fn poll() -> Vec<Temperature> {
    list_sensors().expect("Sensor poll failed!")
}
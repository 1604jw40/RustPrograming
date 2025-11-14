use crate::dataset::{RawRecord, Sample};

pub fn preprocess_records(records: &[RawRecord]) -> Vec<Sample> {
    if records.is_empty() {
        return Vec::new();
    }

    let humidity_mean = mean(records.iter().filter_map(|r| r.humidity));
    let humidity_filled: Vec<f64> = records
        .iter()
        .map(|r| r.humidity.unwrap_or(humidity_mean))
        .collect();

    let temps: Vec<f64> = records.iter().map(|r| r.temperature).collect();
    let pressures: Vec<f64> = records.iter().map(|r| r.pressure).collect();
    let vibrations: Vec<f64> = records.iter().map(|r| r.vibration).collect();
    let qualities: Vec<f64> = records.iter().map(|r| r.quality).collect();

    let temp_bounds = min_max(&temps);
    let humidity_bounds = min_max(&humidity_filled);
    let pressure_bounds = min_max(&pressures);
    let vibration_bounds = min_max(&vibrations);
    let quality_bounds = min_max(&qualities);

    records
        .iter()
        .zip(humidity_filled.into_iter())
        .map(|(record, humidity)| {
            let features = vec![
                scale(record.temperature, temp_bounds),
                scale(humidity, humidity_bounds),
                scale(record.pressure, pressure_bounds),
                scale(record.vibration, vibration_bounds),
                scale(record.quality, quality_bounds),
            ];

            Sample::new(
                record.id,
                features,
                record.label.map(|label| label.to_string()),
            )
        })
        .collect()
}

fn mean<'a, I>(iter: I) -> f64
where
    I: Iterator<Item = f64>,
{
    let mut total = 0.0;
    let mut count = 0usize;
    for v in iter {
        total += v;
        count += 1;
    }
    if count == 0 {
        0.0
    } else {
        total / count as f64
    }
}

fn min_max(values: &[f64]) -> (f64, f64) {
    let min = values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let max = values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    if min.is_infinite() || max.is_infinite() {
        (0.0, 0.0)
    } else {
        (min, max)
    }
}

fn scale(value: f64, bounds: (f64, f64)) -> f64 {
    let (min, max) = bounds;
    if (max - min).abs() < f64::EPSILON {
        0.0
    } else {
        (value - min) / (max - min)
    }
}

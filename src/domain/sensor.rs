//! Sensor 도메인 구현
//!
//! - 입력 스키마 정의 (RawRecord)
//! - 전처리 로직 구현 (min-max 정규화 + 파생 피처)
//! - 저장은 공통 Sample 구조(Parquet)로 처리

use log::info;
use rayon::prelude::*;

use crate::domain::{DataDomain, DomainError};
use crate::model::Sample;

/// 입력 스키마 정의
#[derive(Debug, Clone)]
pub struct SensorRecord {
    pub id: String,
    pub temperature: f64,
    pub humidity: Option<f64>,
    pub pressure: f64,
    pub vibration: f64,
    pub quality: f64,
    pub label: Option<String>,
}

/// Sensor 도메인의 전처리기
pub struct SensorDomain;

impl DataDomain for SensorDomain {
    type RawRecord = SensorRecord;

    fn name() -> &'static str { "sensor" }

    fn parse_record(record: &csv::StringRecord) -> Result<Self::RawRecord, DomainError> {
        Ok(SensorRecord {
            id: record.get(0).ok_or(DomainError::MissingField("id"))?.trim().to_string(),
            temperature: parse_required(record.get(1), "temperature")?,
            humidity: parse_optional(record.get(2)),
            pressure: parse_required(record.get(3), "pressure")?,
            vibration: parse_required(record.get(4), "vibration")?,
            quality: parse_required(record.get(5), "quality")?,
            label: record
                .get(6)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string()),
        })
    }

    fn preprocess(records: &[Self::RawRecord]) -> Vec<Sample> {
        preprocess_sensor_records(records)
    }
}

fn preprocess_sensor_records(records: &[SensorRecord]) -> Vec<Sample> {
    if records.is_empty() {
        return Vec::new();
    }

    let stats = PreprocessStats::from_records(records);
    info!(
        "[sensor] stats temp {:?} humidity {:?} pressure {:?} vibration {:?} quality {:?}",
        stats.temp_bounds,
        stats.humidity_bounds,
        stats.pressure_bounds,
        stats.vibration_bounds,
        stats.quality_bounds,
    );

    records
        .par_iter()
        .map(|record| {
            let humidity = record.humidity.unwrap_or(stats.humidity_mean);
            let features = vec![
                scale(record.temperature, stats.temp_bounds),
                scale(humidity, stats.humidity_bounds),
                scale(record.pressure, stats.pressure_bounds),
                scale(record.vibration, stats.vibration_bounds),
                scale(record.quality, stats.quality_bounds),
                scale(record.temperature * humidity, stats.temp_humidity_bounds),
                scale(record.pressure * record.vibration, stats.pressure_vibration_bounds),
            ];
            Sample::new(record.id.clone(), features, record.label.clone())
        })
        .collect()
}

struct PreprocessStats {
    humidity_mean: f64,
    temp_bounds: (f64, f64),
    humidity_bounds: (f64, f64),
    pressure_bounds: (f64, f64),
    vibration_bounds: (f64, f64),
    quality_bounds: (f64, f64),
    temp_humidity_bounds: (f64, f64),
    pressure_vibration_bounds: (f64, f64),
}

impl PreprocessStats {
    fn from_records(records: &[SensorRecord]) -> Self {
        let humidity_values: Vec<f64> = records.iter().filter_map(|r| r.humidity).collect();
        let humidity_mean = if humidity_values.is_empty() {
            0.0
        } else {
            humidity_values.iter().sum::<f64>() / humidity_values.len() as f64
        };

        let filled_humidity: Vec<f64> = records
            .iter()
            .map(|r| r.humidity.unwrap_or(humidity_mean))
            .collect();

        let temps: Vec<f64> = records.iter().map(|r| r.temperature).collect();
        let pressures: Vec<f64> = records.iter().map(|r| r.pressure).collect();
        let vibrations: Vec<f64> = records.iter().map(|r| r.vibration).collect();
        let qualities: Vec<f64> = records.iter().map(|r| r.quality).collect();
        let temp_x_humidity: Vec<f64> = records
            .iter()
            .zip(filled_humidity.iter())
            .map(|(r, h)| r.temperature * h)
            .collect();
        let pressure_x_vibration: Vec<f64> = records
            .iter()
            .map(|r| r.pressure * r.vibration)
            .collect();

        Self {
            humidity_mean,
            temp_bounds: min_max(&temps),
            humidity_bounds: min_max(&filled_humidity),
            pressure_bounds: min_max(&pressures),
            vibration_bounds: min_max(&vibrations),
            quality_bounds: min_max(&qualities),
            temp_humidity_bounds: min_max(&temp_x_humidity),
            pressure_vibration_bounds: min_max(&pressure_x_vibration),
        }
    }
}

fn min_max(values: &[f64]) -> (f64, f64) {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !min.is_finite() || !max.is_finite() {
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

fn parse_required(field: Option<&str>, name: &'static str) -> Result<f64, DomainError> {
    let raw = field.ok_or(DomainError::MissingField(name))?.trim();
    raw.parse::<f64>()
        .map_err(|_| DomainError::ParseFloat(name.to_string()))
}

fn parse_optional(field: Option<&str>) -> Option<f64> {
    field
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<f64>().ok())
}

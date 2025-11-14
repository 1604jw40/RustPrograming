use csv::ReaderBuilder;

const RAW_DATA: &str = include_str!("../data/raw_dataset.csv");

#[derive(Debug, Clone)]
pub struct RawRecord {
    pub id: String,
    pub temperature: f64,
    pub humidity: Option<f64>,
    pub pressure: f64,
    pub vibration: f64,
    pub quality: f64,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub id: String,
    pub features: Vec<f64>,
    pub label: Option<String>,
}

impl Sample {
    pub fn new(id: impl Into<String>, features: Vec<f64>, label: Option<String>) -> Self {
        Self { id: id.into(), features, label }
    }
}

pub fn expanded_raw_dataset() -> Vec<RawRecord> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(RAW_DATA.as_bytes());

    reader
        .records()
        .filter_map(|row| row.ok())
        .map(|row| RawRecord {
            id: row.get(0).unwrap_or("").trim().to_string(),
            temperature: parse_required(row.get(1)),
            humidity: parse_optional(row.get(2)),
            pressure: parse_required(row.get(3)),
            vibration: parse_required(row.get(4)),
            quality: parse_required(row.get(5)),
            label: row
                .get(6)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string()),
        })
        .collect()
}

fn parse_required(field: Option<&str>) -> f64 {
    field
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0)
}

fn parse_optional(field: Option<&str>) -> Option<f64> {
    field
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<f64>().ok())
}

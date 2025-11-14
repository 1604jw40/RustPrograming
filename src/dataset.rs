#[derive(Debug, Clone)]
pub struct RawRecord {
    pub id: &'static str,
    pub temperature: f64,
    pub humidity: Option<f64>,
    pub pressure: f64,
    pub vibration: f64,
    pub quality: f64,
    pub label: Option<&'static str>,
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
    vec![
        RawRecord { id: "S001", temperature: 12.5, humidity: Some(35.0), pressure: 1.0, vibration: 0.30, quality: 0.88, label: Some("cat") },
        RawRecord { id: "S002", temperature: 15.5, humidity: Some(45.0), pressure: 1.2, vibration: 0.40, quality: 0.92, label: Some("dog") },
        RawRecord { id: "S003", temperature: 7.0, humidity: None, pressure: 0.8, vibration: 0.25, quality: 0.60, label: None },
        RawRecord { id: "S004", temperature: 21.0, humidity: Some(65.0), pressure: 1.4, vibration: 0.55, quality: 0.74, label: Some("bird") },
        RawRecord { id: "S005", temperature: 30.0, humidity: Some(50.0), pressure: 1.5, vibration: 0.80, quality: 0.52, label: Some("cat") },
        RawRecord { id: "S006", temperature: 3.0, humidity: Some(25.0), pressure: 0.9, vibration: 0.20, quality: 0.48, label: None },
        RawRecord { id: "S007", temperature: 26.5, humidity: None, pressure: 1.3, vibration: 0.70, quality: 0.82, label: Some("fox") },
        RawRecord { id: "S008", temperature: 18.0, humidity: Some(58.0), pressure: 1.1, vibration: 0.33, quality: 0.66, label: Some("dog") },
        RawRecord { id: "S009", temperature: 8.5, humidity: Some(32.0), pressure: 0.95, vibration: 0.27, quality: 0.57, label: None },
        RawRecord { id: "S010", temperature: 22.0, humidity: Some(68.0), pressure: 1.6, vibration: 0.62, quality: 0.90, label: Some("bird") },
    ]
}

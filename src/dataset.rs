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
        RawRecord { id: "S011", temperature: 10.0, humidity: Some(40.0), pressure: 1.05, vibration: 0.35, quality: 0.72, label: Some("cat") },
        RawRecord { id: "S012", temperature: 27.0, humidity: Some(60.0), pressure: 1.2, vibration: 0.65, quality: 0.85, label: Some("dog") },
        RawRecord { id: "S013", temperature: 5.0, humidity: None, pressure: 0.75, vibration: 0.18, quality: 0.45, label: None },
        RawRecord { id: "S014", temperature: 19.5, humidity: Some(52.0), pressure: 1.3, vibration: 0.50, quality: 0.78, label: Some("fox") },
        RawRecord { id: "S015", temperature: 24.0, humidity: Some(66.0), pressure: 1.4, vibration: 0.73, quality: 0.83, label: Some("bird") },
        RawRecord { id: "S016", temperature: 14.0, humidity: Some(38.0), pressure: 0.98, vibration: 0.29, quality: 0.58, label: None },
        RawRecord { id: "S017", temperature: 2.0, humidity: Some(20.0), pressure: 0.7, vibration: 0.15, quality: 0.32, label: None },
        RawRecord { id: "S018", temperature: 28.0, humidity: Some(70.0), pressure: 1.55, vibration: 0.82, quality: 0.95, label: Some("cat") },
        RawRecord { id: "S019", temperature: 6.5, humidity: Some(28.0), pressure: 0.85, vibration: 0.24, quality: 0.49, label: None },
        RawRecord { id: "S020", temperature: 23.0, humidity: Some(64.0), pressure: 1.45, vibration: 0.60, quality: 0.88, label: Some("dog") },
    ]
}

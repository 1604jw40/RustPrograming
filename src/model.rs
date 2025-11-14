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

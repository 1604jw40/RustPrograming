use serde::{Deserialize, Serialize};
use serde::de::Error as SerdeDeError;
use crate::{Identifiable, DbError};


#[derive(Debug, Clone, Serialize, Deserialize)]
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


impl Identifiable for Sample {
fn id(&self) -> &str { &self.id }
}


/// CSV 헤더
pub fn csv_headers() -> impl Iterator<Item = &'static str> {
["id", "features", "label"].into_iter()
}


/// Sample → CSV 레코드
pub fn sample_to_record(s: &Sample) -> csv::StringRecord {
let feats = s.features.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
let label = s.label.clone().unwrap_or_default();
csv::StringRecord::from(vec![s.id.clone(), feats, label])
}


/// CSV 레코드 → Sample (공백/콤마 구분)
pub fn record_to_sample(rec: csv::StringRecord) -> Result<Sample, DbError> {
let id = rec.get(0).unwrap_or("").to_string();
let feats_str = rec.get(1).unwrap_or("");
let label = rec.get(2).map(|s| if s.is_empty() { None } else { Some(s.to_string()) }).flatten();


let features: Vec<f64> = feats_str
.split(|c| c == ' ' || c == ',')
.filter(|t| !t.trim().is_empty())
.map(|t| t.trim().parse::<f64>())
.collect::<Result<Vec<_>, _>>()
.map_err(|e| DbError::Json(SerdeDeError::custom(e.to_string())))?;


Ok(Sample { id, features, label })
}
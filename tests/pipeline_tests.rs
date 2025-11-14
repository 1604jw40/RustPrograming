use sampledb::config::PipelineConfig;
use sampledb::dataset::load_raw_records;
use sampledb::domain::sensor::SensorDomain;
use sampledb::model::Sample;
use sampledb::preprocess::preprocess_records;

#[test]
fn config_and_preprocess_smoke() {
    let config = PipelineConfig::load("config/pipeline.toml").expect("config loads");
    let records = load_raw_records::<SensorDomain>(&config).expect("dataset loads");
    assert!(!records.is_empty(), "dataset should not be empty");

    let samples: Vec<Sample> = preprocess_records::<SensorDomain>(&records);
    assert_eq!(samples.len(), records.len());
}

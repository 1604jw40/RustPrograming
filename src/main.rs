use log::{info, warn};
use sampledb::config::PipelineConfig;
use sampledb::dataset::{chunk_records, load_raw_records};
use sampledb::domain::sensor::SensorDomain;
use sampledb::preprocess::preprocess_records;
use sampledb::storage::save_samples_to_parquet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = PipelineConfig::load("config/pipeline.toml")?;
    info!("pipeline stage: configuration loaded");

    let raw_records = load_raw_records::<SensorDomain>(&config)?;
    info!("pipeline stage: parsing complete ({} rows)", raw_records.len());

    let chunked = chunk_records(&raw_records, config.chunk_size);
    for (idx, chunk) in chunked.iter().enumerate() {
        info!("chunk {} -> {} rows", idx, chunk.len());
    }
    info!("pipeline stage: chunk linkage complete");

    if raw_records.is_empty() {
        warn!("no data found; skipping export");
        return Ok(());
    }

    let samples = preprocess_records::<SensorDomain>(&raw_records);
    info!("pipeline stage: preprocessing complete ({} rows)", samples.len());

    save_samples_to_parquet(&samples, &config.output_parquet)?;
    info!("pipeline stage: export complete (saved to {})", config.output_parquet.display());
    Ok(())
}

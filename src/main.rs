use sampledb::dataset::expanded_raw_dataset;
use sampledb::preprocess::preprocess_records;
use sampledb::storage::{save_samples_to_csv, save_samples_to_parquet, StorageError};

fn main() -> Result<(), StorageError> {
    let raw_records = expanded_raw_dataset();
    let samples = preprocess_records(&raw_records);
    println!("preprocessed rows: {}", samples.len());

    save_samples_to_csv(&samples, "processed_samples.csv")?;
    save_samples_to_parquet(&samples, "processed_samples.parquet")?;

    println!("saved CSV + Parquet at project root");
    Ok(())
}

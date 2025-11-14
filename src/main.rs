mod dataset;
mod preprocess;
mod storage;

use crate::dataset::expanded_raw_dataset;
use crate::preprocess::preprocess_records;
use crate::storage::{save_samples_to_parquet, StorageError};

fn main() -> Result<(), StorageError> {
    let raw_records = expanded_raw_dataset();
    let samples = preprocess_records(&raw_records);
    println!("preprocessed rows: {}", samples.len());

    save_samples_to_parquet(&samples, "processed_samples.parquet")?;

    println!("saved Parquet dataset for downstream modeling");
    Ok(())
}

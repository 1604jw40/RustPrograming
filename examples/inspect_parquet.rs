use std::error::Error;

use sampledb::storage::load_samples_from_parquet;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "processed_samples.parquet".to_string());

    let samples = load_samples_from_parquet(&path)?;
    println!("parquet path: {}", path);
    println!("total rows: {}", samples.len());

    for (idx, sample) in samples.iter().take(5).enumerate() {
        println!("#{} {:?}", idx, sample);
    }

    Ok(())
}

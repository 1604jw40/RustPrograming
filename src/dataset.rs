use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use csv::ReaderBuilder;
use glob::glob;
use log::{info, warn};
use thiserror::Error;

use crate::config::PipelineConfig;
use crate::domain::{DataDomain, DomainError};

#[derive(Debug, Error)]
pub enum DatasetError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("csv error: {0}")]
    Csv(#[from] csv::Error),

    #[error("glob pattern error: {0}")]
    Glob(#[from] glob::PatternError),

    #[error("domain parse error: {0}")]
    Domain(#[from] DomainError),
}

pub fn load_raw_records<D: DataDomain>(config: &PipelineConfig) -> Result<Vec<D::RawRecord>, DatasetError> {
    let pattern = config.input_dir.join(&config.file_pattern);
    let pattern_str = pattern.to_string_lossy().into_owned();
    info!(
        "[{}] stage:start parse -> pattern {}",
        D::name(),
        pattern_str
    );

    let mut all_records = Vec::new();
    for entry in glob(&pattern_str)? {
        match entry {
            Ok(path) => {
                let count_before = all_records.len();
                read_single_file::<D>(path.clone(), &mut all_records)?;
                info!(
                    "[{}] parsed {} rows from {}",
                    D::name(),
                    all_records.len() - count_before,
                    path.display()
                );
            },
            Err(err) => warn!("failed to read path from glob: {}", err),
        }
    }
    Ok(all_records)
}

fn read_single_file<D: DataDomain>(path: PathBuf, sink: &mut Vec<D::RawRecord>) -> Result<(), DatasetError> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    for record in csv_reader.records() {
        let record = record?;
        sink.push(D::parse_record(&record)?);
    }

    Ok(())
}

pub fn chunk_records<T>(records: &[T], chunk_size: usize) -> Vec<&[T]> {
    records.chunks(chunk_size.max(1)).collect()
}

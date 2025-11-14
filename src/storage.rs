use std::fs::File;
use std::path::Path;

use polars::prelude::*;
use thiserror::Error;

use crate::dataset::Sample;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("polars error: {0}")]
    Polars(#[from] polars::error::PolarsError),
}

pub fn save_samples_to_parquet<P: AsRef<Path>>(samples: &[Sample], path: P) -> Result<(), StorageError> {
    if samples.is_empty() {
        File::create(path)?;
        return Ok(());
    }

    let mut df = samples_to_df(samples)?;
    let file = File::create(path)?;
    ParquetWriter::new(file).finish(&mut df)?;
    Ok(())
}

pub fn load_samples_from_parquet<P: AsRef<Path>>(path: P) -> Result<Vec<Sample>, StorageError> {
    let file = File::open(path)?;
    let df = ParquetReader::new(file).finish()?;
    dataframe_to_samples(df)
}

fn samples_to_df(samples: &[Sample]) -> PolarsResult<DataFrame> {
    let ids: Vec<String> = samples.iter().map(|s| s.id.clone()).collect();
    let labels: Vec<Option<String>> = samples.iter().map(|s| s.label.clone()).collect();
    let total_feature_len: usize = samples.iter().map(|s| s.features.len()).sum();

    let mut list_builder = ListPrimitiveChunkedBuilder::<Float64Type>::new(
        "features".into(),
        samples.len(),
        total_feature_len,
        DataType::Float64,
    );

    for sample in samples {
        list_builder.append_slice(&sample.features);
    }

    let mut features_series = list_builder.finish().into_series();
    _ = features_series.rename("features".into());

    DataFrame::new(vec![
        Column::new("id".into(), ids),
        Column::new("label".into(), labels),
        Column::new("features".into(), features_series),
    ])
}

fn dataframe_to_samples(df: DataFrame) -> Result<Vec<Sample>, StorageError> {
    let id_series = df.column("id")?.as_materialized_series().str()?;
    let label_series = df.column("label")?.as_materialized_series().str()?;
    let features_column = df.column("features")?.as_materialized_series().list()?;

    let mut samples = Vec::with_capacity(df.height());
    for idx in 0..df.height() {
        let id = id_series
            .get(idx)
            .ok_or_else(|| PolarsError::NoData("missing id column".into()))?;

        let label = label_series.get(idx).map(|val| val.to_string());
        let feature_series = features_column
            .get_as_series(idx)
            .ok_or_else(|| PolarsError::NoData("missing features column".into()))?;
        let feature_values = feature_series
            .f64()
            .map_err(|_| PolarsError::SchemaMismatch("features column must be f64".into()))?
            .into_iter()
            .flatten()
            .collect::<Vec<f64>>();

        samples.push(Sample::new(id, feature_values, label));
    }
    Ok(samples)
}

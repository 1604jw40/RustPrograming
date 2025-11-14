pub mod sensor;

use csv::StringRecord;
use thiserror::Error;

use crate::model::Sample;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("parse float error: {0}")]
    ParseFloat(String),
}

pub trait DataDomain {
    type RawRecord: Send + Sync;

    /// 도메인 식별자 (로그 등에서 사용)
    fn name() -> &'static str;

    /// 입력 스키마 정의: CSV 레코드를 RawRecord 로 변환
    fn parse_record(record: &StringRecord) -> Result<Self::RawRecord, DomainError>;

    /// 전처리 로직 구현: RawRecord 슬라이스를 받아 Sample 리스트로 변환
    fn preprocess(records: &[Self::RawRecord]) -> Vec<Sample>;
}

use crate::domain::DataDomain;
use crate::model::Sample;

/// 전처리 로직 구현 지점
///
/// - 도메인별 전처리기는 `DataDomain::preprocess` 에 구현한다.
/// - 이 함수는 공통 파이프라인에서 해당 전처리를 호출하는 역할만 맡는다.
pub fn preprocess_records<D: DataDomain>(records: &[D::RawRecord]) -> Vec<Sample> {
    D::preprocess(records)
}

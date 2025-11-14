# Sensor Pipeline Demo

확장된 임시 센서 데이터셋을 전처리한 뒤 CSV/Parquet 두 가지 포맷으로 저장/재적재할 수 있는 경량 파이프라인입니다. `main.rs` 는 저장 단계만 실행하고, 재적재는 `storage` 모듈의 API 로 확인할 수 있습니다.

## 모듈 구성
- `data/raw_dataset.csv`  
  20개의 센서 샘플이 정의된 원본 데이터 파일. 빈 문자열은 결측 습도를 의미합니다.
- `src/dataset.rs`  
  `expanded_raw_dataset()` 가 위 CSV 를 `include_str!` 로 읽어 `RawRecord` 목록으로 파싱합니다.
- `src/preprocess.rs`  
  결측 습도를 평균으로 대체하고, 다섯 개 수치 항목을 Min-Max 정규화하여 `Sample` 벡터를 만듭니다.
- `src/storage.rs`  
  `Sample` 벡터를 CSV 혹은 Parquet 로 저장/재적재하는 헬퍼. CSV 는 공백 구분 feature 문자열을 사용하고, Parquet 은 Polars DataFrame + list 컬럼을 통해 저장합니다.

## 실행 흐름 (`main.rs`)
1. `expanded_raw_dataset()` 으로 CSV 기반 로우 데이터셋 확보.
2. `preprocess_records()` 로 결측 보정 + 정규화 → 최종 feature vector 작성.
3. `save_samples_to_csv` 와 `save_samples_to_parquet` 으로 각각 `processed_samples.csv` / `processed_samples.parquet` 생성.

```bash
cargo run
```

실행 후 프로젝트 루트에 두 결과 파일이 생성되며, 같은 이름의 파일이 있을 경우 자동으로 덮어씁니다.

## 재적재 확인
`storage::load_samples_from_csv` 또는 `storage::load_samples_from_parquet` 을 호출하면 저장된 데이터를 다시 `Vec<Sample>` 형태로 얻을 수 있습니다. 필요 시 별도 바이너리/테스트에서 호출하여 파이프라인을 검증하세요.

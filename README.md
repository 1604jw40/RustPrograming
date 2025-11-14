# Sensor Pipeline Demo

확장된 임시 센서 데이터셋을 전처리한 뒤 Parquet 포맷으로 내보내어 Python 감성 분석/모델링 파이프라인과 바로 연동할 수 있게 만든 경량 ETL 입니다. `main.rs` 는 전처리 + Parquet 저장까지만 실행하며, 필요 시 다른 바이너리에서 재적재 함수를 호출할 수 있습니다.

## 모듈 구성
- `data/raw_dataset.csv`  
  20개의 센서 샘플이 정의된 원본 데이터 파일. 빈 문자열은 결측 습도를 의미합니다.
- `src/dataset.rs`  
  `expanded_raw_dataset()` 가 위 CSV 를 `include_str!` 로 읽어 `RawRecord` 목록으로 파싱합니다.
- `src/preprocess.rs`  
  결측 습도를 평균으로 대체하고, 다섯 개 수치 항목을 Min-Max 정규화하여 `Sample` 벡터를 만듭니다.
- `src/storage.rs`  
  `Sample` 벡터를 Parquet 로 저장/재적재하는 헬퍼. Polars DataFrame 의 list 컬럼을 이용해 feature vector 를 보존합니다.

## 실행 흐름 (`main.rs`)
1. `expanded_raw_dataset()` 으로 CSV 기반 로우 데이터셋 확보.
2. `preprocess_records()` 로 결측 보정 + 정규화 → 최종 feature vector 작성.
3. `save_samples_to_parquet` 으로 `processed_samples.parquet` 생성 (Python 파이프라인이 그대로 읽을 수 있는 형태).

### 파이프라인 흐름 요약
1. `data/raw_dataset.csv` 에 정의된 센서 샘플을 로드 (`expanded_raw_dataset`).
2. `preprocess_records` 에서
   - 습도 결측값 평균 대체
   - 온도/습도/압력/진동/품질 min-max 정규화
   - 정규화된 5차원 feature vector 와 라벨을 `Sample` 로 구성
3. `storage` 모듈로 Parquet 파일을 생성 (`processed_samples.parquet`)

```bash
cargo run
```

실행 후 프로젝트 루트에 `processed_samples.parquet` 이 생성되며, 같은 이름의 파일이 있을 경우 자동으로 덮어씁니다.

## 재적재 확인
`storage::load_samples_from_parquet` 을 호출하면 저장된 데이터를 다시 `Vec<Sample>` 형태로 얻을 수 있습니다. 필요 시 별도 바이너리/테스트에서 호출하여 파이프라인을 검증하세요.

# Sensor Pipeline Demo

확장된 센서 데이터셋을 전처리해 Parquet 포맷으로 저장하고, Python 감성 분석/모델링 파이프라인에 곧장 전달하는 용도로 만든 경량 ETL입니다. Rust 측에서는 설정 파일을 통해 입력 소스·청크 크기·출력 경로 등을 제어하고, 병렬 전처리와 구조화된 로깅을 지원합니다. 이 문서는 전체 파이프라인의 동작 방식, 모듈 간 역할, 그리고 확장 시 주의점을 상세히 설명합니다.

---

## 1. 구성 개요

| 모듈/파일 | 역할 |
|-----------|------|
| `config/pipeline.toml` | 파이프라인 전반의 설정값(입력 디렉터리, glob 패턴, chunk size, 출력 Parquet 경로)을 정의합니다. |
| `data/*.csv` | 원시 센서 샘플이 담긴 파일 집합입니다. 동일 스키마를 가진 CSV 파일을 여러 개 두고 glob 패턴으로 한 번에 읽습니다. |
| `src/config.rs` | TOML 구성을 읽어 `PipelineConfig` 구조체로 변환합니다. chunk 사이즈가 0이면 자동으로 1로 보정해 안정성을 확보합니다. |
| `src/domain/mod.rs` | 입력 스키마·전처리 로직·저장 규칙을 캡슐화하기 위한 `DataDomain` 트레이트를 제공합니다. |
| `src/domain/sensor.rs` | Sensor 도메인 구현 예시입니다. RawRecord 정의, 전처리 로직, 파생 피처 생성이 모두 이 파일에 모여 있습니다. |
| `src/model.rs` | 공통 `Sample` 구조체 (id + feature vector + label)를 정의합니다. |
| `src/dataset.rs` | glob 패턴에 매칭되는 모든 CSV를 스트리밍으로 읽어 `DataDomain::RawRecord` 벡터로 통합하고, chunk 정보를 계산합니다. 파일별 레코드 수를 로깅해 추적성을 높입니다. |
| `src/preprocess.rs` | 도메인별 전처리 함수를 호출하는 래퍼입니다. 파이프라인 코드는 `DataDomain::preprocess` 를 호출하기만 하면 됩니다. |
| `src/storage.rs` | Polars DataFrame을 통해 `Sample` 벡터를 Parquet로 저장/재적재합니다. 리스트 컬럼을 사용해 가변 길이 feature vector를 보존합니다. |
| `src/main.rs` | 설정 로드 → 데이터 로드/청크 로깅 → 병렬 전처리 → Parquet 저장 순으로 파이프라인을 실행하며, `env_logger` 기반의 로그를 출력합니다. |
| `tests/pipeline_tests.rs` | 구성 파일과 전처리 함수가 정상 동작하는지 확인하는 smoke 테스트입니다. |

---

## 2. 설정 파일 (`pipeline.toml`)

```toml
input_dir = "data"
file_pattern = "*.csv"
output_parquet = "processed_samples.parquet"
chunk_size = 8
```

- `input_dir`: CSV 파일이 위치한 디렉터리 경로.
- `file_pattern`: glob 패턴. 예를 들어 `*.csv` 혹은 `raw/*.txt` 식으로 조정할 수 있습니다.
- `output_parquet`: 완성된 feature dataset 을 저장할 파일 경로.
- `chunk_size`: 청크 단위 로깅/처리를 위해 사용하는 기본 크기. 0 이하 값은 자동으로 1로 보정됩니다.

구성 파일을 바꾸면 데이터 소스나 출력 경로를 코드 수정 없이 즉시 변경할 수 있습니다.

---

## 3. 데이터 로더 (`dataset`)

1. `load_raw_records::<Domain>(&config)`는 `input_dir/file_pattern` 조합으로 glob 검색을 수행하여 CSV 파일 리스트를 얻습니다.
2. 각 파일은 `BufReader + csv::ReaderBuilder`로 스트리밍 파싱되며, `Domain::parse_record` 를 통해 RawRecord 로 변환됩니다.
3. 전처리/로그를 위해 `chunk_records(&records, chunk_size)`로 청크 정보를 계산합니다. chunk 크기가 매우 크거나 작으면 config만 수정하면 됩니다.

**유의사항**
- Sensor 도메인 구현에서는 CSV의 빈 문자열을 결측치로 간주합니다. 다른 도메인을 추가할 경우 `parse_record` 내부에서 원하는 검증 로직을 구현하세요.
- 숫자 파싱 실패 시 `DomainError::ParseFloat`로 실패를 알리므로, 파이프라인을 즉시 중단하거나 로깅 후 무시하는 등 정책을 쉽게 바꿀 수 있습니다.

---

## 4. 전처리 (`domain::<D>` + `preprocess`)

- `DataDomain::preprocess(records)`에 도메인별 전처리 로직을 작성합니다. Sensor 도메인은 습도 평균 대체 → min-max 정규화 → 파생 피처(온도×습도, 압력×진동) 생성 → Rayon 병렬 처리 순으로 구성돼 있습니다.
- `src/preprocess.rs`는 공통 파이프라인에서 `D::preprocess`를 호출하는 래퍼에 불과하므로, 새로운 도메인을 추가할 때도 파이프라인 코드를 손댈 필요가 없습니다.
- 도메인 모듈 상단에 “입력 스키마/전처리/저장” 주석을 넣어 어떤 부분을 수정해야 하는지 빠르게 파악할 수 있습니다.

---

## 5. 저장/재적재 (`storage`)

- `save_samples_to_parquet(samples, path)`  
  Polars DataFrame을 사용해 `id`, `label`, `features(list<f64>)` 세 컬럼으로 구성한 뒤 Parquet Writer로 저장합니다. 빈 데이터셋이면 빈 파일을 생성합니다.

- `load_samples_from_parquet(path)`  
  동일한 스키마를 역직렬화하여 `Vec<Sample>`을 복원합니다. Python 측에서 feature list를 별도 파싱할 경우, Arrow/Polars를 이용해 리스트 컬럼을 그대로 읽을 수 있습니다.

에러는 `StorageError`를 통해 IO와 Polars 오류만 래핑하므로, 호출 측에서 단일 Result 타입으로 간편하게 다룰 수 있습니다.

---

## 6. 실행 흐름 (`main.rs`)

1. `env_logger::init()`으로 로그 시스템을 초기화합니다.
2. `PipelineConfig::load`로 설정을 읽습니다.
3. `load_raw_records`가 glob 패턴에 맞는 모든 CSV를 읽어 통합합니다.
4. `chunk_records`로 청크 정보를 계산하고, 각 청크 크기를 로그로 남겨 모니터링합니다.
5. 데이터가 비어 있으면 경고 로그를 출력하고 종료합니다.
6. `preprocess_records::<SensorDomain>`로 해당 도메인의 병렬 전처리를 실행합니다.
7. `save_samples_to_parquet`으로 최종 Parquet 파일을 생성합니다.

```bash
cargo run
```

위 명령어를 실행하면 `config/pipeline.toml` 설정에 따라 데이터를 처리하고, 완료 후 `processed_samples.parquet`을 출력합니다.

---

## 7. 로깅 & 검증

- 로그 레벨은 `RUST_LOG=info cargo run`처럼 환경 변수로 제어할 수 있습니다.
- 병렬 처리 구간에서 통계를 한 번만 계산하도록 설계되어 있으므로, 로그는 chunk/통계/저장 단계 중심으로 출력됩니다.
- `tests/pipeline_tests.rs`는 구성 및 전처리 로직이 깨지지 않았는지 상시 확인하는 스모크 테스트입니다. 필요 시 여기에 재적재 검증, 대용량 샘플 등에 대한 케이스를 추가할 수 있습니다.

---

## 8. 확장 아이디어

- **스트리밍/청크 업로드**: 현재는 한 번에 모든 레코드를 메모리에 담지만, chunk 기반으로 저장을 분할하거나 스트리밍 처리 구조를 추가할 수 있습니다.
- **새로운 도메인 추가**: `src/domain` 에 새로운 모듈을 만들고 `DataDomain` 을 구현하면 교통, 로그, 텍스트 등 다른 스키마도 동일한 파이프라인으로 처리할 수 있습니다.
- **고급 특징 추출**: 텍스트 기반 감성 데이터를 다룬다면 형태소 분석, n-gram, TF-IDF, 임베딩 등으로 feature 생성 로직을 확장하세요.
- **벤치마크/프로파일링**: Criterion을 도입해 전처리 병목을 측정하거나, feature 생성 단계를 SIMD/특화 라이브러리로 가속할 수 있습니다.
- **파이썬과의 교차 검증**: Parquet를 pandas/Polars에서 읽어 모델에 투입하고, 추론 결과를 다시 Rust 파이프라인으로 넘기는 형태의 round-trip을 구성할 수 있습니다.

---

## 9. 요약

- **목표**: 다중 CSV 입력 → 병렬 전처리 → Parquet 저장 → Python 분석.
- **핵심 기술**: glob 기반 데이터 로딩, Rayon 병렬화, Polars Parquet I/O, 구성 기반 제어, 통합 테스트.
- **결과물**: `processed_samples.parquet` 하나로 감성 분석/모델링 파이프라인에 즉시 연동 가능.

필요한 경우 추가 구성 옵션이나 전처리 단계, 저장 포맷을 자유롭게 확장할 수 있습니다. Rust 특유의 안정성과 성능을 활용해 대규모 ETL을 안전하게 운영할 수 있도록 설계되었으며, `DataDomain` 트레이트를 통해 하나의 프로젝트 안에서 여러 데이터 스키마를 쉽게 공존시킬 수 있습니다.

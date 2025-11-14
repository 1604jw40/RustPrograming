# Rust-Specific Notes

1. **구성 기반 데이터 로딩**  
   - `pipeline.toml` 에서 입력 디렉터리/패턴을 읽고 `glob` 로 여러 CSV 파일을 탐색합니다. 런타임 IO 를 사용하므로 데이터 양이 커져도 바이너리 크기가 증가하지 않습니다.
   - 각 CSV 는 `ReaderBuilder` + `BufReader` 로 스트리밍 처리합니다.

2. **소유권/복제 패턴**  
   - `domain::<D>::RawRecord` 와 `Sample` 모두 `String` 을 보유합니다. 전처리에서 `Sample::new(record.id.clone(), …)` 를 호출하는 이유는, RawRecord 의 필드를 그대로 넘기면 원본 데이터셋의 소유권을 이동하게 되어 이후 반복에서 사용할 수 없기 때문입니다.
   - `Option<String>` 을 `clone()` 할 때 `Option` 자체는 가벼운 enum 이지만 내부 `String` 은 힙 데이터를 복사합니다. 라벨이 누락된 경우 `None` 이라 비용이 없고, 라벨이 있는 경우에만 힙 복제가 발생합니다.

3. **`DataDomain` 트레이트**  
   - 입력 스키마·전처리 로직을 모듈별로 분리하기 위해 `DataDomain` 트레이트를 사용합니다. `parse_record` 와 `preprocess` 만 구현하면 동일 파이프라인으로 다양한 데이터 종류를 처리할 수 있습니다.
   - Sensor 도메인 외에 교통 데이터 등을 추가할 경우 `src/domain/<새도메인>.rs` 에 RawRecord/전처리를 구현하고, `main.rs` 에서 `load_raw_records::<새도메인>` 형식으로 호출하면 됩니다.

4. **Parallel iterator 전처리**  
   - `records.par_iter()` 를 사용해 Rayon 기반 병렬화를 적용했습니다. 전처리 로직은 순수 함수여야 하므로, 공유 상태 없이 feature vector 만 생성하도록 구성되어 있습니다.

5. **Polars + 리스트 컬럼 구성**  
   - `ListPrimitiveChunkedBuilder::<Float64Type>::new("features".into(), len, total, DataType::Float64)` 를 통해 list 컬럼을 만듭니다. builder 는 내부적으로 `MutablePrimitiveArray` 를 쓰므로, `append_slice(&sample.features)` 호출 시 데이터가 복사됩니다.
   - `Series::new("id".into(), ids)` 처럼 `PlSmallStr` 로 이름을 넘겨야 하므로 `into()`를 사용합니다. Polars 0.44 이후 `DataFrame::new` 는 `Column` 타입을 기대하므로 `Column::new(name, values)` 호출이 필요합니다.

6. **에러 모델링**  
   - `ConfigError`, `DatasetError`, `StorageError` 가 각각 `thiserror` 를 통해 `std::error::Error` 구현을 제공하므로, `main` 에서는 `Box<dyn Error>`로 단순히 전파할 수 있습니다.

7. **파일 저장 시 덮어쓰기**  
   - `File::create(path)?` 는 기존 파일이 있으면 truncate 후 재작성합니다. 실행 전에 수동 삭제할 필요가 없고, `Ok(())` 반환으로 종료 시점에 파일이 완전히 기록된 상태를 보장합니다 (에러 발생 시 즉시 Propagate).

8. **파이썬 파이프라인 연동**  
   - 최종 산출물은 `config`에서 지정한 위치의 Parquet 파일 하나이며, Python 의 pandas/Polars 등에서 그대로 읽어 후속 감성 분석 단계를 실행할 수 있습니다. Rust 쪽에서는 저장만 담당하고 재적재가 필요하면 `load_samples_from_parquet` 을 직접 호출합니다.

9. **검증/테스트**  
   - `tests/pipeline_tests.rs` 가 설정 로딩 + 전처리 스모크 테스트를 수행합니다. 필요한 경우 Criterion 등을 추가해 벤치마크도 쉽게 붙일 수 있습니다.

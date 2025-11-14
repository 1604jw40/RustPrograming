# Rust-Specific Notes

1. **`include_str!` + CSV 파서**  
   - `include_str!("../data/raw_dataset.csv")` 로 빌드 시점에 파일을 문자열 리터럴로 포함합니다. 런타임 파일 IO 가 아니므로, 수정 후에는 `cargo clean` 없이도 `cargo run` 한 번으로 갱신 확인이 가능합니다.
   - `csv::ReaderBuilder::from_reader(RAW_DATA.as_bytes())` 를 통해 문자열 Slice 를 곧장 CSV 리더에 넘깁니다. 외부 파일 핸들 대신 바이너리 버퍼를 쓰는 패턴이라 라이프타임 이슈가 없습니다.

2. **소유권/복제 패턴**  
   - `RawRecord` 와 `Sample` 모두 `String` 을 보유하게 변경했습니다. 전처리에서 `Sample::new(record.id.clone(), …, record.label.clone())` 를 호출하는 이유는, `RawRecord` 의 필드를 그대로 넘기면 원본 데이터셋의 소유권을 이동하게 되어 이후 반복에서 사용할 수 없기 때문입니다.
   - `Option<String>` 을 `clone()` 할 때 `Option` 자체는 가벼운 enum 이지만 내부 `String` 은 힙 데이터를 복사합니다. 라벨이 누락된 경우 `None` 이라 비용이 없고, 라벨이 있는 경우에만 힙 복제가 발생합니다.

3. **Iterator 기반 전처리**  
   - `mean(records.iter().filter_map(|r| r.humidity))` 처럼 iterator 체인을 함수에 넘깁니다. 여기서 `filter_map` 은 `Option<f64>` 를 `f64` 로 추출하므로 소유권이 아닌 값 복사만 발생합니다.
   - `zip` + `map` 체인을 사용해 전처리 전체를 불변 슬라이스 대상으로 수행합니다. 빌림 규칙상, 같은 반복에서 mutable borrow 를 섞을 수 없으므로, 미리 `humidity_filled` 를 생성한 뒤 `zip` 으로 묶어 사용합니다.

4. **Polars + 리스트 컬럼 구성**  
   - `ListPrimitiveChunkedBuilder::<Float64Type>::new("features".into(), len, total, DataType::Float64)` 를 통해 list 컬럼을 만듭니다. builder 는 내부적으로 `MutablePrimitiveArray` 를 쓰므로, `append_slice(&sample.features)` 호출 시 데이터가 복사됩니다.
   - `Series::new("id".into(), ids)` 처럼 `PlSmallStr` 로 이름을 넘겨야 하므로 `into()`를 사용합니다. Polars 0.44 이후 `DataFrame::new` 는 `Column` 타입을 기대하므로 `Column::new(name, values)` 호출이 필요합니다.

5. **에러 모델링**  
   - `storage::StorageError` 는 `thiserror::Error` 를 이용해 CSV/Polars/IO 에러를 하나의 enum 으로 래핑합니다. `?` 사용 시 `From` 트레이트로 자동 변환되므로, 함수 시그니처에서 `Result<_, StorageError>` 를 유지할 수 있습니다.

6. **파일 저장 시 덮어쓰기**  
   - `File::create(path)?` 는 기존 파일이 있으면 truncate 후 재작성합니다. 실행 전에 수동 삭제할 필요가 없고, `Ok(())` 반환으로 종료 시점에 파일이 완전히 기록된 상태를 보장합니다 (에러 발생 시 즉시 Propagate).

use std::collections::HashMap;
use thiserror::Error;

/// 모든 엔티티는 고유 문자열 ID를 가져야 함
pub trait Identifiable {
    fn id(&self) -> &str;
}

/// DB 에러 정의
#[derive(Debug, Error)]
pub enum DbError {
    #[error("duplicate id: {0}")]
    Duplicate(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("csv error: {0}")]
    Csv(#[from] csv::Error),

    #[error("serde json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("parse float error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
}

/// 제네릭 인메모리 DB
pub struct LibDB<T: Identifiable> {
    store: HashMap<String, T>,
}

impl<T: Identifiable> LibDB<T> {
    #[inline]
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    /// 중복 키 방지 insert
    pub fn insert(&mut self, item: T) -> Result<(), DbError> {
        let key = item.id().to_string();
        if self.store.contains_key(&key) {
            return Err(DbError::Duplicate(key));
        }
        self.store.insert(key, item);
        Ok(())
    }

    /// 존재하면 replace, 없으면 insert
    #[inline]
    pub fn upsert(&mut self, item: T) {
        let key = item.id().to_string();
        self.store.insert(key, item);
    }

    /// 읽기 전용 대여
    #[inline]
    pub fn get(&self, id: &str) -> Option<&T> {
        self.store.get(id)
    }

    /// 가변 대여
    #[inline]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        self.store.get_mut(id)
    }

    /// 제거 → 소유권 반환
    #[inline]
    pub fn remove(&mut self, id: &str) -> Option<T> {
        self.store.remove(id)
    }

    #[inline]
    pub fn len(&self) -> usize { self.store.len() }

    #[inline]
    pub fn is_empty(&self) -> bool { self.store.is_empty() }

    /// 읽기 전용 iterator
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.store.values()
    }

    // -------------------------
    // CSV / JSONL I/O
    // -------------------------

    /// CSV 로드 (사용자: csv::StringRecord → T 변환 제공)
    pub fn load_csv<R, F>(&mut self, reader: R, mut to_item: F) -> Result<usize, DbError>
    where
        R: std::io::Read,
        F: FnMut(csv::StringRecord) -> Result<T, DbError>,
    {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut count = 0;
        for rec in rdr.records() {
            let rec = rec?;
            let item = to_item(rec)?;
            self.upsert(item);
            count += 1;
        }
        Ok(count)
    }

    /// CSV 저장
    pub fn save_csv<W, I, H, F>(&self, mut writer: W, headers: H, mut to_record: F) -> Result<(), DbError>
    where
        W: std::io::Write,
        I: IntoIterator<Item = &'static str>,
        H: Fn() -> I,
        F: FnMut(&T) -> csv::StringRecord,
    {
        let mut wtr = csv::Writer::from_writer(&mut writer);
        wtr.write_record(headers())?;

        for item in self.store.values() {
            let rec = to_record(item);
            wtr.write_record(&rec)?;
        }

        wtr.flush()?;
        Ok(())
    }

    /// JSONL 로드 (한 줄 = 한 객체)
    pub fn load_jsonl<R>(&mut self, reader: R) -> Result<usize, DbError>
    where
        R: std::io::BufRead,
        T: serde::de::DeserializeOwned,
    {
        let mut count = 0;
        for line in reader.lines() {
            let line = line?;             // io error → DbError::Io
            if line.trim().is_empty() { continue; }
            let item: T = serde_json::from_str(&line)?; // json parse → DbError::Json
            self.upsert(item);
            count += 1;
        }
        Ok(count)
    }

    /// JSONL 저장
    pub fn save_jsonl<W>(&self, mut writer: W) -> Result<(), DbError>
    where
        W: std::io::Write,
        T: serde::Serialize,
    {
        for item in self.store.values() {
            let line = serde_json::to_string(item)?; // serialize error → DbError::Json
            writeln!(writer, "{}", line)?;
        }
        Ok(())
    }
}

pub mod sample;

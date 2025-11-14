use std::fs::File;
use std::io::{BufRead, BufReader};

// 같은 crate 내부에서는 crate:: 로 접근하는 게 정석
use sampledb::{LibDB, DbError};
use sampledb::sample::{Sample, csv_headers, sample_to_record, record_to_sample};


fn main() -> Result<(), DbError> {
// 1) DB 생성 및 삽입
let mut db = LibDB::<Sample>::new();
db.insert(Sample::new("S001", vec![0.1, 0.2, 0.3], Some("cat".into())))?;
db.upsert(Sample::new("S002", vec![1.0, 2.0], None));


// 2) 가변 대여로 수정
if let Some(s) = db.get_mut("S002") { s.features.push(3.0); }


// 3) JSONL 저장 → 로드
let mut jsonl = File::create("samples.jsonl")?;
db.save_jsonl(&mut jsonl)?;


let reader = BufReader::new(File::open("samples.jsonl")?);
let mut db2 = LibDB::<Sample>::new();
let loaded = db2.load_jsonl(reader)?;
println!("loaded from jsonl: {} records", loaded);


// 4) CSV 저장 → 로드
let mut csvf = File::create("samples.csv")?;
db2.save_csv(&mut csvf, || csv_headers(), sample_to_record)?;


let csv_input = std::fs::read("samples.csv")?;
let mut db3 = LibDB::<Sample>::new();
let count = db3.load_csv(&csv_input[..], record_to_sample)?;
println!("loaded from csv: {} records", count);


// 5) 이터레이션
for s in db3.iter() { println!("{:?}", s); }


Ok(())
}
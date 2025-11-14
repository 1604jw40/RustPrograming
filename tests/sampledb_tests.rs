use sampledb::{LibDB};
use sampledb::sample::Sample;


#[test]
fn insert_get_remove_roundtrip() {
let mut db = LibDB::<Sample>::new();
db.insert(Sample::new("ID1", vec![1.0, 2.0], Some("A".into()))).unwrap();


let r = db.get("ID1").unwrap();
assert_eq!(r.label.as_deref(), Some("A"));


let out = db.remove("ID1").unwrap(); // 소유권 이동
assert_eq!(out.features.len(), 2);
assert!(db.get("ID1").is_none());
}


#[test]
fn upsert_replace() {
let mut db = LibDB::<Sample>::new();
db.upsert(Sample::new("ID2", vec![0.1], None));
db.upsert(Sample::new("ID2", vec![9.9, 8.8], Some("B".into())));


let r = db.get("ID2").unwrap();
assert_eq!(r.features, vec![9.9, 8.8]);
assert_eq!(r.label.as_deref(), Some("B"));
}
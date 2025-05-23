use std::fs::File;
use std::collections::HashSet;
use std::io::Write;

use crate::PatientRecord;
use csv::{ReaderBuilder, WriterBuilder};

pub fn merge_files(paths: Vec<&str>, output_path: &str) {
    let mut seen = HashSet::new();
    let mut all_records = Vec::new();

    for path in paths {
        let file = File::open(path).expect("Failed to open input file");
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        for result in rdr.deserialize() {
            let record: PatientRecord = result.expect("Failed to parse record");
            let key = format!("{}{}", record.patient_id, record.date);
            if seen.insert(key) {
                all_records.push(record);
            }
        }
    }

    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_writer(File::create(output_path).expect("Failed to create output file"));

    for record in &all_records {
        wtr.serialize(record).expect("Failed to write record");
    }

    wtr.flush().expect("Failed to flush writer");
    println!(
        "✅ Merged {} records into {}",
        all_records.len(),
        output_path
    );
}

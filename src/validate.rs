use crate::PatientRecord;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

/// Validates a health data file and reports inconsistencies (heart rate = 0, extremely low temperature, etc.)
pub fn run_validation(path: &str) {
    let file = File::open(path).expect("❌ Failed to open file for validation.");
    let reader = BufReader::new(file);
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    let mut record_count = 0;
    let mut issues_found = 0;

    for result in reader.deserialize() {
        let record: PatientRecord = match result {
            Ok(r) => r,
            Err(e) => {
                println!("❗ Invalid row: {}", e);
                issues_found += 1;
                continue;
            }
        };

        record_count += 1;

        if record.heart_rate == 0 || record.temperature < 30.0 {
            println!(
                "⚠️  Suspect data - Patient {} on {}: {:?}",
                record.patient_id, record.date, record
            );
            issues_found += 1;
        }
    }

    println!(
        "✅ Validation complete: {} records checked, {} issue(s) found.",
        record_count, issues_found
    );
}

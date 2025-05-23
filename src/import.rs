use std::fs::File;
use std::path::Path;
use serde_json::Deserializer;
use csv::Reader;
use crate::PatientRecord;

/// Load all records from `path`.  
/// If `fmt == Some("json")` _or_ the file extension is `.json`, parse as JSON — otherwise CSV.
pub fn load_file(path: &str, fmt: Option<&str>) -> Vec<PatientRecord> {
    let is_json = fmt
        .map(|f| f.eq_ignore_ascii_case("json"))
        .unwrap_or_else(|| Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("json"))
            .unwrap_or(false));

    // Open the file
    let file = File::open(path).expect("Failed to open input file");

    if is_json {
        // JSON‐lines: one record per line
        Deserializer::from_reader(file)
            .into_iter::<PatientRecord>()
            .map(|r| r.expect("JSON parse error"))
            .collect()
    } else {
        // CSV
        let mut rdr = Reader::from_reader(file);
        rdr.deserialize()
            .map(|r| r.expect("CSV parse error"))
            .collect()
    }
}

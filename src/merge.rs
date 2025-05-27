use crate::{AktenError, PatientRecord};
use std::fs::{File, OpenOptions};
use std::path::Path;
use csv::{ReaderBuilder, WriterBuilder};
use serde_json;

/// Merges multiple input files into a single output CSV file
pub fn merge_files(inputs: &Vec<&str>, output: &str, medical_mode: bool) -> Result<(), AktenError> {
    let mut all_records: Vec<PatientRecord> = Vec::new();

    for path in inputs {
        if !Path::new(path).exists() {
            return Err(AktenError::InvalidPath(path.to_string()));
        }

        let file = File::open(path).map_err(AktenError::Io)?;

        if path.ends_with(".json") {
            let records: Vec<PatientRecord> = serde_json::from_reader(file).map_err(AktenError::Json)?;
            all_records.extend(records);
        } else if path.ends_with(".csv") {
            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
            for result in rdr.deserialize() {
                let record: PatientRecord = result.map_err(AktenError::Csv)?;
                all_records.push(record);
            }
        } else {
            return Err(AktenError::UnsupportedFormat);
        }
    }

    // Write combined records to output CSV
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output)
        .map_err(AktenError::Io)?;

    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);
    for record in &all_records {
        wtr.serialize(record).map_err(AktenError::Csv)?;
    }
    wtr.flush().map_err(AktenError::Io)?;

    if medical_mode {
        println!("📋 Medical mode enabled – merged {} records to '{}'.", all_records.len(), output);
    } else {
        println!("📦 Merged {} records to '{}'.", all_records.len(), output);
    }

    Ok(())
}
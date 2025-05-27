use crate::{AktenError, PatientRecord};
use std::fs::File;
use csv::WriterBuilder;
use serde_json;

/// Export data in CSV or JSON format
pub fn export_data(records: &Vec<PatientRecord>, format: &str, output: &str, medical_mode: bool) -> Result<(), AktenError> {
    match format.to_lowercase().as_str() {
        "csv" => export_csv(records, output, medical_mode),
        "json" => export_json(records, output, medical_mode),
        _ => Err(AktenError::UnsupportedFormat),
    }
}

/// Export AI-ready JSON (structured for ML pipelines)
pub fn export_ai_data(records: &Vec<PatientRecord>, output: &str) -> Result<(), AktenError> {
    let file = File::create(output).map_err(AktenError::Io)?;
    serde_json::to_writer_pretty(&file, records).map_err(AktenError::Json)?;
    println!("🤖 Exported AI-ready data to '{}'.", output);
    Ok(())
}

fn export_csv(records: &Vec<PatientRecord>, output: &str, medical_mode: bool) -> Result<(), AktenError> {
    let file = File::create(output).map_err(AktenError::Io)?;
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);

    for record in records {
        wtr.serialize(record).map_err(AktenError::Csv)?;
    }
    wtr.flush().map_err(AktenError::Io)?;

    if medical_mode {
        println!("🩺 Medical-mode CSV export complete: {} records to '{}'.", records.len(), output);
    } else {
        println!("📄 CSV export complete: {} records to '{}'.", records.len(), output);
    }
    Ok(())
}

fn export_json(records: &Vec<PatientRecord>, output: &str, medical_mode: bool) -> Result<(), AktenError> {
    let file = File::create(output).map_err(AktenError::Io)?;
    serde_json::to_writer_pretty(&file, records).map_err(AktenError::Json)?;

    if medical_mode {
        println!("🩺 Medical-mode JSON export complete: {} records to '{}'.", records.len(), output);
    } else {
        println!("📄 JSON export complete: {} records to '{}'.", records.len(), output);
    }
    Ok(())
}

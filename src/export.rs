use crate::{AktenError, PatientRecord};
use std::fs::File;
use csv::WriterBuilder;
use serde::{Serialize}; // ✅ Fix missing macro for #[derive(Serialize)]
use serde_json;
use chrono::Utc;

/// Export data in supported formats (CSV/JSON)
pub fn export_data(
    records: &[PatientRecord],
    format: &str,
    output_path: &str,
    medical_mode: bool,
) -> Result<(), AktenError> {
    match format.to_lowercase().as_str() {
        "csv" => export_csv(records, output_path, medical_mode),
        "json" => export_json(records, output_path, medical_mode),
        _ => Err(AktenError::UnsupportedFormat),
    }
}

/// Export AI-ready JSON with additional metadata
pub fn export_ai_data(
    records: &[PatientRecord],
    output_path: &str,
) -> Result<(), AktenError> {
    #[derive(Serialize)]
    struct AiExportRecord<'a> {
        record: &'a PatientRecord,
        metadata: AiMetadata,
    }

    #[derive(Serialize)]
    struct AiMetadata {
        export_timestamp: String,
        schema_version: &'static str,
    }

    let file = File::create(output_path)?;
    let records_with_metadata: Vec<AiExportRecord> = records.iter().map(|record| AiExportRecord {
        record,
        metadata: AiMetadata {
            export_timestamp: Utc::now().to_rfc3339(),
            schema_version: "1.0",
        },
    }).collect();

    serde_json::to_writer_pretty(file, &records_with_metadata)?;

    println!("🤖 Exported {} AI-ready records to '{}'", records.len(), output_path);
    Ok(())
}

/// CSV export implementation
fn export_csv(
    records: &[PatientRecord],
    output_path: &str,
    medical_mode: bool,
) -> Result<(), AktenError> {
    let file = File::create(output_path)?;
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .flexible(medical_mode) // Allow variable columns in medical mode
        .from_writer(file);

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;

    let mode_prefix = if medical_mode { "🩺 Medical" } else { "📄 Standard" };
    println!(
        "{} CSV export complete: {} records to '{}'",
        mode_prefix,
        records.len(),
        output_path
    );
    Ok(())
}

/// JSON export implementation
fn export_json(
    records: &[PatientRecord],
    output_path: &str,
    medical_mode: bool,
) -> Result<(), AktenError> {
    let file = File::create(output_path)?;

    if medical_mode {
        #[derive(Serialize)]
        struct MedicalRecordExport<'a> {
            patient_data: &'a PatientRecord,
            clinical_notes: String,
        }

        let enhanced_records: Vec<MedicalRecordExport> = records.iter().map(|record| MedicalRecordExport {
            patient_data: record,
            clinical_notes: String::new(), // Placeholder for actual notes
        }).collect();

        serde_json::to_writer_pretty(file, &enhanced_records)?;
    } else {
        serde_json::to_writer_pretty(file, records)?;
    }

    let mode_prefix = if medical_mode { "🩺 Medical" } else { "📄 Standard" };
    println!(
        "{} JSON export complete: {} records to '{}'",
        mode_prefix,
        records.len(),
        output_path
    );
    Ok(())
}

use crate::{AktenError, PatientRecord};
use std::fs::File;
use std::io::Write;
use serde::Serialize;
use serde_json;

/// Risk output structure to serialize for JSON export
#[derive(Serialize)]
pub struct RiskResult {
    pub patient_id: u32,
    pub date: String,
    pub risks: Vec<String>,
    pub heart_rate: u32,
    pub bp_systolic: u32,
    pub bp_diastolic: u32,
    pub temperature: f32,
    pub blood_sugar: f32,
}

/// Rule-based risk prediction printed to terminal
pub fn predict_risks(records: &[PatientRecord]) -> Result<(), AktenError> {
    let mut flagged = vec![];

    for record in records {
        let risks = detect_risks(record);
        if !risks.is_empty() {
            flagged.push((record, risks));
        }
    }

    if flagged.is_empty() {
        println!("✅ No immediate health risks detected.");
    } else {
        println!("⚠️ Risk Summary:");
        for (record, risks) in flagged {
            println!(
                "Patient {} on {}: {:?} => HR: {}, BP: {}/{}, Temp: {}C, Sugar: {}",
                record.patient_id,
                record.date,
                risks,
                record.heart_rate,
                record.bp_systolic,
                record.bp_diastolic,
                record.temperature,
                record.blood_sugar
            );
        }
    }

    Ok(())
}

/// Export prediction results as JSON to a file
pub fn export_risks_as_json(records: &[PatientRecord], output_path: &str) -> Result<(), AktenError> {
    let mut results = vec![];

    for record in records {
        let risks = detect_risks(record);
        if !risks.is_empty() {
            results.push(RiskResult {
                patient_id: record.patient_id,
                date: record.date.clone(),
                risks,
                heart_rate: record.heart_rate,
                bp_systolic: record.bp_systolic,
                bp_diastolic: record.bp_diastolic,
                temperature: record.temperature,
                blood_sugar: record.blood_sugar,
            });
        }
    }

    if results.is_empty() {
        println!("✅ No risks found, no JSON exported.");
        return Ok(());
    }

    let json = serde_json::to_string_pretty(&results)?;
    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;

    println!("🧠 Exported {} risk prediction records to `{}`", results.len(), output_path);
    Ok(())
}

/// Core logic for detecting patient risk indicators
fn detect_risks(record: &PatientRecord) -> Vec<String> {
    let mut risks = vec![];

    if record.heart_rate > 100 {
        risks.push("High heart rate".to_string());
    }
    if record.bp_systolic > 140 || record.bp_diastolic > 90 {
        risks.push("High blood pressure".to_string());
    }
    if record.temperature > 38.0 {
        risks.push("Fever".to_string());
    }
    if record.blood_sugar > 7.0 {
        risks.push("High blood sugar".to_string());
    }

    risks
}

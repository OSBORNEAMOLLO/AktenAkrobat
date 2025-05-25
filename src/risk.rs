use crate::PatientRecord;
use std::fs::File;
use std::io::Write;
use serde::Serialize;

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

/// Rule-based prediction printed to screen
pub fn predict_risks(records: &[PatientRecord]) {
    let mut flagged = vec![];

    for record in records {
        let mut risks = vec![];

        if record.heart_rate > 100 {
            risks.push("High heart rate");
        }
        if record.bp_systolic > 140 || record.bp_diastolic > 90 {
            risks.push("High blood pressure");
        }
        if record.temperature > 38.0 {
            risks.push("Fever");
        }
        if record.blood_sugar > 7.0 {
            risks.push("High blood sugar");
        }

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
}

/// Export prediction results as JSON to a file
pub fn export_risks_as_json(records: &[PatientRecord], output_path: &str) {
    let mut results = vec![];

    for record in records {
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
        return;
    }

    let json = serde_json::to_string_pretty(&results).expect("Failed to serialize risk JSON");
    let mut file = File::create(output_path).expect("Failed to create risk output JSON");
    file.write_all(json.as_bytes()).expect("Failed to write risk JSON");

    println!("🧠 Exported {} risk prediction records to `{}`", results.len(), output_path);
}

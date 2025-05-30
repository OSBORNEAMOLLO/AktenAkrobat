use crate::{AktenError, PatientRecord};
use crate::config::ThresholdConfig;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

/// Risk output structure for JSON export
#[derive(Debug, Serialize)]
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

/// Predict health risks from patient records
pub fn predict_risks(
    records: &[PatientRecord],
    config: &ThresholdConfig,
) -> Result<(), AktenError> {
    let mut flagged = vec![];

    for record in records {
        let risks = detect_risks(record, &config.thresholds);
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
                "Patient {} on {}: {:?} => HR: {}, BP: {}/{}, Temp: {:.1}°C, Sugar: {:.1}",
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

/// Export risk predictions as JSON
pub fn export_risks_as_json(
    records: &[PatientRecord],
    config: &ThresholdConfig,
    output_path: &str,
) -> Result<(), AktenError> {
    let mut results = vec![];

    for record in records {
        let risks = detect_risks(record, &config.thresholds);
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

    let json = serde_json::to_string_pretty(&results)?;
    File::create(output_path)?.write_all(json.as_bytes())?;
    
    println!("✅ Exported risk predictions to {}", output_path);
    Ok(())
}

/// Core risk detection logic
fn detect_risks(record: &PatientRecord, thresholds: &crate::config::Thresholds) -> Vec<String> {
    let mut risks = vec![];

    if record.heart_rate < thresholds.heart_rate.min 
        || record.heart_rate > thresholds.heart_rate.max {
        risks.push("Abnormal heart rate".to_string());
    }
    if record.bp_systolic >= thresholds.blood_pressure.systolic
        || record.bp_diastolic >= thresholds.blood_pressure.diastolic {
        risks.push("Hypertensive crisis".to_string());
    }
    if record.temperature > thresholds.fever {
        risks.push("Fever".to_string());
    }
    if record.blood_sugar > thresholds.hyperglycemia {
        risks.push("Hyperglycemia".to_string());
    }
    if record.blood_sugar < thresholds.hypoglycemia {
        risks.push("Hypoglycemia".to_string());
    }

    risks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Thresholds, CriticalHr, HypertensiveCrisis};

    fn test_thresholds() -> Thresholds {
        Thresholds {
            heart_rate: CriticalHr { min: 60, max: 100 },
            blood_pressure: HypertensiveCrisis { systolic: 140, diastolic: 90 },
            hypothermia: 35.0,
            fever: 38.0,
            hypoglycemia: 3.9,
            hyperglycemia: 7.0,
        }
    }

    #[test]
    fn test_detect_risks() {
        let thresholds = test_thresholds();
        let normal_record = PatientRecord {
            patient_id: 1,
            date: "2023-01-01".to_string(),
            heart_rate: 75,
            bp_systolic: 120,
            bp_diastolic: 80,
            temperature: 37.0,
            blood_sugar: 5.5,
            steps: 0,
        };
        assert!(detect_risks(&normal_record, &thresholds).is_empty());
    }
}
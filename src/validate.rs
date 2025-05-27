use crate::{AktenError, PatientRecord};
use std::{fs::File, path::Path};
use csv::ReaderBuilder;
use serde_json;

#[derive(Debug)]
pub struct ValidationResult {
    pub record_count: u32,
    pub issues_found: u32,
    pub critical_alerts: Vec<String>,
}

/// Validates health data with medical-grade checks
pub fn run_validation(path: &str, medical_mode: bool) -> Result<ValidationResult, AktenError> {
    if !Path::new(path).exists() {
        return Err(AktenError::InvalidPath(path.to_string()));
    }

    let file = File::open(path)?;
    let mut result = ValidationResult {
        record_count: 0,
        issues_found: 0,
        critical_alerts: Vec::new(),
    };

    if path.ends_with(".json") {
        let records: Vec<PatientRecord> = serde_json::from_reader(file)?;
        validate_records(&records, medical_mode, &mut result);
    } else {
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        for record in rdr.deserialize() {
            let record: PatientRecord = record?;
            validate_record(&record, medical_mode, &mut result);
            result.record_count += 1;
        }
    }

    Ok(result)
}

fn validate_records(records: &[PatientRecord], medical_mode: bool, result: &mut ValidationResult) {
    for record in records {
        validate_record(record, medical_mode, result);
        result.record_count += 1;
    }
}

fn validate_record(record: &PatientRecord, medical_mode: bool, result: &mut ValidationResult) {
    // Basic validation (applies to all modes)
    if record.heart_rate == 0 {
        log_issue(record, "Zero heart rate", result);
    }

    if record.temperature < 30.0 || record.temperature > 42.0 {
        log_issue(record, "Abnormal body temperature", result);
    }

    // Medical-mode specific checks
    if medical_mode {
        if record.bp_systolic > 180 || record.bp_diastolic > 120 {
            log_issue(record, "Hypertensive crisis", result);
        }

        if record.blood_sugar > 400.0 {
            log_issue(record, "Critical hyperglycemia", result);
        }
    }
}

fn log_issue(record: &PatientRecord, message: &str, result: &mut ValidationResult) {
    let alert = format!(
        "⚠️ {} - Patient {} ({}): HR={}, Temp={}, BP={}/{}",
        message,
        record.patient_id,
        record.date,
        record.heart_rate,
        record.temperature,
        record.bp_systolic,
        record.bp_diastolic
    );

    result.issues_found += 1;
    result.critical_alerts.push(alert);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_csv(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_validation() {
        let csv_data = "\
patient_id,date,heart_rate,bp_systolic,bp_diastolic,temperature,blood_sugar,steps
1,2023-01-01,0,120,80,29.0,100,5000";

        let file = create_test_csv(csv_data);
        let result = run_validation(file.path().to_str().unwrap(), false).unwrap();

        assert_eq!(result.issues_found, 2); // Zero HR + low temp
    }
}

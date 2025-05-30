use crate::{AktenError, PatientRecord, config::ThresholdConfig};
use csv::ReaderBuilder;
use rayon::prelude::*;
use serde_json;
use std::{fs::File, path::Path, sync::Mutex};
use tracing::{info, warn};

/// Results container for validation operations
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub record_count: usize,
    pub issues_found: usize,
    pub critical_alerts: Vec<String>,
    pub warnings: Vec<String>,
}

/// Main validation entry point
pub fn run_validation(
    input_path: &str,
    medical_mode: bool,
    config: &ThresholdConfig,
) -> Result<ValidationResult, AktenError> {
    let path = input_path.trim();
    validate_path(path)?;

    let records = load_records(path)?;
    let result = validate_records(&records, medical_mode, config);

    info!("Validated {} records - {} issues found", 
          result.record_count, 
          result.issues_found);
    
    Ok(result)
}

/// Validate file path existence and format
fn validate_path(path: &str) -> Result<(), AktenError> {
    match path {
        "" => Err(AktenError::InvalidPath("Empty path provided".into())),
        _ if !Path::new(path).exists() => Err(AktenError::InvalidPath(path.into())),
        _ if !path.ends_with(".json") && !path.ends_with(".csv") => {
            Err(AktenError::UnsupportedFormat)
        }
        _ => Ok(()),
    }
}

/// Load records from supported file formats
fn load_records(path: &str) -> Result<Vec<PatientRecord>, AktenError> {
    let file = File::open(path)?;
    
    if path.ends_with(".json") {
        serde_json::from_reader(file).map_err(Into::into)
    } else {
        ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(file)
            .deserialize()
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

/// Parallel record validation
fn validate_records(
    records: &[PatientRecord],
    medical_mode: bool,
    config: &ThresholdConfig,
) -> ValidationResult {
    let result = Mutex::new(ValidationResult::default());

    records.par_iter().for_each(|record| {
        let mut guard = result.lock().unwrap();
        guard.record_count += 1;
        validate_record(record, medical_mode, &mut guard, config);
    });

    result.into_inner().unwrap()
}

/// Individual record validation logic
fn validate_record(
    record: &PatientRecord,
    medical_mode: bool,
    result: &mut ValidationResult,
    config: &ThresholdConfig,
) {
    check_vital_signs(record, result, config);
    
    if medical_mode {
        check_medical_conditions(record, result, config);
    }
}

/// Core vital sign validation
fn check_vital_signs(
    record: &PatientRecord,
    result: &mut ValidationResult,
    config: &ThresholdConfig,
) {
    let thresholds = &config.thresholds;

    // Heart rate check
    if record.heart_rate < thresholds.heart_rate.min 
        || record.heart_rate > thresholds.heart_rate.max 
    {
        log_alert(
            record, 
            &format!("Abnormal HR ({} bpm)", record.heart_rate),
            true,
            result
        );
    }

    // Temperature check
    if record.temperature < thresholds.hypothermia {
        log_alert(
            record,
            &format!("Hypothermia ({:.1}°C)", record.temperature),
            true,
            result
        );
    } else if record.temperature > thresholds.fever {
        log_alert(
            record,
            &format!("Fever ({:.1}°C)", record.temperature),
            true,
            result
        );
    }
}

/// Medical-specific condition checks
fn check_medical_conditions(
    record: &PatientRecord,
    result: &mut ValidationResult,
    config: &ThresholdConfig,
) {
    let thresholds = &config.thresholds;

    // Blood pressure evaluation
    match (record.bp_systolic, record.bp_diastolic) {
        (s, d) if s >= thresholds.blood_pressure.systolic 
               || d >= thresholds.blood_pressure.diastolic => {
            log_alert(record, "Hypertensive crisis", true, result);
        }
        (s, d) if s >= 140 || d >= 90 => {
            log_alert(record, "Stage 1/2 hypertension", false, result);
        }
        _ => {}
    }

    // Blood sugar evaluation
    if record.blood_sugar > thresholds.hyperglycemia {
        log_alert(record, "Hyperglycemia", true, result);
    } else if record.blood_sugar < thresholds.hypoglycemia {
        log_alert(record, "Hypoglycemia", false, result);
    }
}

/// Unified alert logging
fn log_alert(
    record: &PatientRecord,
    message: &str,
    is_critical: bool,
    result: &mut ValidationResult,
) {
    let alert = if is_critical {
        result.critical_alerts.push(format!(
            "🚨 CRITICAL: {} | Patient {} ({})\n   HR: {}, Temp: {:.1}°C, BP: {}/{}",
            message,
            record.patient_id,
            record.date,
            record.heart_rate,
            record.temperature,
            record.bp_systolic,
            record.bp_diastolic
        ));
        "CRITICAL"
    } else {
        result.warnings.push(format!(
            "⚠️ WARNING: {} | Patient {}",
            message,
            record.patient_id
        ));
        "WARNING"
    };

    result.issues_found += 1;
    warn!("{} alert for patient {}: {}", alert, record.patient_id, message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn test_config() -> ThresholdConfig {
        ThresholdConfig {
            thresholds: crate::config::Thresholds {
                heart_rate: crate::config::CriticalHr { min: 40, max: 140 },
                blood_pressure: crate::config::HypertensiveCrisis { 
                    systolic: 180, 
                    diastolic: 120 
                },
                hypothermia: 35.0,
                fever: 38.0,
                hypoglycemia: 70.0,
                hyperglycemia: 400.0,
            },
        }
    }

    #[test]
    fn test_normal_record() -> Result<(), AktenError> {
        let csv_data = "\
patient_id,date,heart_rate,bp_systolic,bp_diastolic,temperature,blood_sugar,steps
1,2023-01-01,72,120,80,36.5,90,5000";
        
        let file = NamedTempFile::new()?;
        std::fs::write(&file, csv_data)?;
        
        let result = run_validation(file.path().to_str().unwrap(), true, &test_config())?;
        assert_eq!(result.issues_found, 0);
        Ok(())
    }

    #[test]
    fn test_critical_alerts() -> Result<(), AktenError> {
        let csv_data = "\
patient_id,date,heart_rate,bp_systolic,bp_diastolic,temperature,blood_sugar,steps
1,2023-01-01,180,190,110,39.0,450,0";
        
        let file = NamedTempFile::new()?;
        std::fs::write(&file, csv_data)?;
        
        let result = run_validation(file.path().to_str().unwrap(), true, &test_config())?;
        assert_eq!(result.critical_alerts.len(), 4);
        assert_eq!(result.issues_found, 4);
        Ok(())
    }
}
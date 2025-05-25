use crate::PatientRecord;

/// Struct to hold features for AI-readiness (numerical input format)
#[derive(Debug)]
pub struct RiskFeatures {
    pub heart_rate: u32,
    pub bp_systolic: u32,
    pub bp_diastolic: u32,
    pub temperature: f32,
    pub blood_sugar: f32,
}

/// Checks for basic health risks based on simple thresholds.
/// Also extracts risk features for future ML model consumption.
pub fn predict_risks(records: &[PatientRecord]) {
    let mut flagged = vec![];
    let mut feature_set: Vec<RiskFeatures> = vec![];

    for record in records {
        // Collect features in AI-ready format
        feature_set.push(RiskFeatures {
            heart_rate: record.heart_rate,
            bp_systolic: record.bp_systolic,
            bp_diastolic: record.bp_diastolic,
            temperature: record.temperature,
            blood_sugar: record.blood_sugar,
        });

        // Apply rule-based logic
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

    // Output summary
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

    // 🧠 Optional debug preview of the AI-ready data structure
    println!("\n📦 Extracted Features (AI-ready):");
    for (i, f) in feature_set.iter().enumerate() {
        println!(
            "- Row {} => HR: {}, BP: {}/{}, Temp: {}, Sugar: {}",
            i + 1,
            f.heart_rate,
            f.bp_systolic,
            f.bp_diastolic,
            f.temperature,
            f.blood_sugar
        );
    }
}

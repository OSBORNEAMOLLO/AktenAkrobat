use crate::{AktenError, PatientRecord};

/// Summarizes health metrics from a dataset
pub fn summarize_data(records: &[PatientRecord], medical_mode: bool) -> Result<(), AktenError> {
    if records.is_empty() {
        println!("📭 No records found to summarize.");
        return Ok(());
    }

    let count = records.len() as f32;
    let avg_heart_rate: f32 = records.iter().map(|r| r.heart_rate as f32).sum::<f32>() / count;
    let avg_bp_systolic: f32 = records.iter().map(|r| r.bp_systolic as f32).sum::<f32>() / count;
    let avg_bp_diastolic: f32 = records.iter().map(|r| r.bp_diastolic as f32).sum::<f32>() / count;
    let avg_temperature: f32 = records.iter().map(|r| r.temperature).sum::<f32>() / count;
    let avg_blood_sugar: f32 = records.iter().map(|r| r.blood_sugar).sum::<f32>() / count;
    let total_steps: u32 = records.iter().map(|r| r.steps).sum();

    println!("📊 Summary ({} records):", records.len());
    println!("- Avg Heart Rate: {:.1} bpm", avg_heart_rate);
    println!("- Avg Blood Pressure: {:.0}/{:.0} mmHg", avg_bp_systolic, avg_bp_diastolic);
    println!("- Avg Temperature: {:.1} °C", avg_temperature);
    println!("- Avg Blood Sugar: {:.1} mmol/L", avg_blood_sugar);
    println!("- Total Steps: {}", total_steps);

    if medical_mode {
        println!("🩺 Medical Mode: Additional metrics or annotations may be added here.");
    }

    Ok(())
}

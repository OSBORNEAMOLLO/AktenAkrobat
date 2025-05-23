use crate::PatientRecord;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

pub fn summarize_data(path: &str) {
    let file = File::open(path).expect("Failed to open file");
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut count = 0;
    let mut sum_hr = 0;
    let mut sum_temp = 0.0;
    let mut sum_steps = 0;

    for result in reader.deserialize() {
        let record: PatientRecord = result.expect("Failed to parse");
        sum_hr += record.heart_rate;
        sum_temp += record.temperature;
        sum_steps += record.steps;
        count += 1;
    }

    if count > 0 {
        println!("📊 Summary Report:");
        println!("• Total records: {}", count);
        println!("• Avg heart rate: {:.1} bpm", sum_hr as f32 / count as f32);
        println!("• Avg temperature: {:.1}°C", sum_temp / count as f32);
        println!("• Total steps: {}", sum_steps);
    } else {
        println!("⚠️ No records found.");
    }
}

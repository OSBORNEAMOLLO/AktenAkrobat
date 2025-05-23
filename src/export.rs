use std::fs::File;
use crate::PatientRecord;
use csv::WriterBuilder;
use serde_json;

/// Exports patient records to CSV or JSON
pub fn export_data(records: &Vec<PatientRecord>, format: &str, output_path: &str) {
    match format.to_lowercase().as_str() {
        "csv" => {
            let mut wtr = WriterBuilder::new()
                .has_headers(true)
                .from_writer(File::create(output_path).expect("Failed to create CSV file"));
            for record in records {
                wtr.serialize(record).expect("Failed to write record");
            }
            wtr.flush().expect("Failed to flush CSV writer");
            println!("✅ Exported {} records to CSV at `{}`", records.len(), output_path);
        }

        "json" => {
            let file = File::create(output_path).expect("Failed to create JSON file");
            serde_json::to_writer_pretty(file, &records).expect("Failed to write JSON");
            println!("✅ Exported {} records to JSON at `{}`", records.len(), output_path);
        }

        _ => {
            println!("❌ Unsupported format: {}. Use 'csv' or 'json'.", format);
        }
    }
}

mod validate;
mod summarize;
mod merge;
mod export;
mod risk;

use std::fs::File;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;
use serde_json;

/// A CLI tool for loading, validating, merging, exporting, predicting, and summarizing patient health data.
#[derive(Parser)]
#[command(name = "AktenAkrobat")]
#[command(about = "A CLI tool for health data integration and analysis (CSV + JSON)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Load a CSV or JSON file of patient records
    LoadFile {
        #[arg(short, long)]
        path: String,
    },

    /// Validate health data file
    Validate {
        #[arg(short, long)]
        path: String,
    },

    /// Summarize health data
    Summarize {
        #[arg(short, long)]
        path: String,
    },

    /// Merge multiple data files
    MergeFiles {},

    /// Export merged data as CSV or JSON
    Export {
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        output: String,
    },

    /// Export data formatted for AI use
    ExportAi {
        #[arg(short, long)]
        output: String,
    },

    /// Predict risk (prints to screen)
    PredictRisk {},

    /// Predict risk and export as JSON
    PredictRiskJson {
        #[arg(short, long)]
        output: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientRecord {
    pub patient_id: u32,
    pub date: String,
    pub heart_rate: u32,
    pub bp_systolic: u32,
    pub bp_diastolic: u32,
    pub temperature: f32,
    pub blood_sugar: f32,
    pub steps: u32,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::LoadFile { path } => {
            if path.to_lowercase().ends_with(".json") {
                let file = File::open(path).expect("Failed to open JSON file");
                let records: Vec<PatientRecord> =
                    serde_json::from_reader(file).expect("Failed to parse JSON");
                println!("✅ Loaded {} records from JSON", records.len());
                for record in records {
                    println!("{:?}", record);
                }
            } else {
                let file = File::open(path).expect("Failed to open CSV file");
                let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
                let mut count = 0;
                for result in rdr.deserialize() {
                    let record: PatientRecord = result.expect("CSV deserialize failed");
                    println!("{:?}", record);
                    count += 1;
                }
                println!("✅ Loaded {} records from CSV", count);
            }
        }

        Commands::Validate { path } => {
            println!("🔍 Validating data at `{}`...", path);
            validate::run_validation(&path);
        }

        Commands::Summarize { path } => {
            println!("📊 Summarizing data at `{}`...", path);
            summarize::summarize_data(&path);
        }

        Commands::MergeFiles {} => {
            println!("🔗 Merging mock_data/patients_sample.csv with another_sample.csv...");
            merge::merge_files(
                vec![
                    "mock_data/patients_sample.csv",
                    "mock_data/another_sample.csv",
                ],
                "mock_data/merged_output.csv",
            );
        }

        Commands::Export { format, output } => {
            let path = "mock_data/merged_output.csv";
            let file = File::open(path).expect("Failed to open source file");
            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
            let mut records = Vec::new();
            for result in rdr.deserialize() {
                let record: PatientRecord = result.expect("CSV deserialize failed");
                records.push(record);
            }
            export::export_data(&records, format, output);
        }

        Commands::ExportAi { output } => {
            let path = "mock_data/merged_output.csv";
            let file = File::open(path).expect("Failed to open source file");
            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
            let mut records = Vec::new();
            for result in rdr.deserialize() {
                let record: PatientRecord = result.expect("CSV deserialize failed");
                records.push(record);
            }
            export::export_ai_data(&records, output);
        }

        Commands::PredictRisk {} => {
            let path = "mock_data/merged_output.csv";
            let file = File::open(path).expect("Failed to open source file");
            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
            let mut records = Vec::new();
            for result in rdr.deserialize() {
                let record: PatientRecord = result.expect("CSV deserialize failed");
                records.push(record);
            }
            println!("🤖 Predicting risk (basic rules)...");
            risk::predict_risks(&records);
        }

        Commands::PredictRiskJson { output } => {
            let path = "mock_data/merged_output.csv";
            let file = File::open(path).expect("Failed to open source file");
            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
            let mut records = Vec::new();
            for result in rdr.deserialize() {
                let record: PatientRecord = result.expect("CSV deserialize failed");
                records.push(record);
            }
            println!("🧠 Exporting prediction results as JSON...");
            risk::export_risks_as_json(&records, output);
        }
    }
}

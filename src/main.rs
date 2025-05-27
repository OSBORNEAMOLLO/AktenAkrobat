mod validate;
mod summarize;
mod merge;
mod export;
mod risk;

use std::{fs::File, path::Path};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use csv::ReaderBuilder;
use thiserror::Error;

/// Custom error type for AktenAkrobat
#[derive(Debug, Error)]
pub enum AktenError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Unsupported file format (must be .csv or .json)")]
    UnsupportedFormat,
}

/// Patient health record structure
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

/// CLI interface definition
#[derive(Parser)]
#[command(name = "AktenAkrobat")]
#[command(about = "Health Data CLI Toolkit (CSV/JSON)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable medical-specific processing (DICOM/HL7 aware)
    #[arg(long)]
    medical_mode: bool,

    /// Perform a dry run without changing files
    #[arg(long)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate health data file
    Validate {
        #[arg(short, long)]
        path: String,
    },

    /// Summarize health metrics
    Summarize {
        #[arg(short, long)]
        path: String,
    },

    /// Merge multiple data files
    MergeFiles {
        #[arg(short, long)]
        output: String,
    },

    /// Export data (CSV/JSON)
    Export {
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        output: String,
    },

    /// Export AI-ready JSON
    ExportAi {
        #[arg(short, long)]
        output: String,
    },

    /// Predict health risks
    PredictRisk {
        #[arg(short, long)]
        path: String,
    },

    /// Export predicted risks to a JSON file
    ExportRiskJson {
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        output: String,
    },
}

/// Validates file path and extension
fn validate_file_path(path: &str) -> Result<(), AktenError> {
    if !Path::new(path).exists() {
        return Err(AktenError::InvalidPath(path.to_string()));
    }

    if !(path.ends_with(".csv") || path.ends_with(".json")) {
        return Err(AktenError::UnsupportedFormat);
    }

    Ok(())
}

/// Loads patient records from file
fn load_records(path: &str) -> Result<Vec<PatientRecord>, AktenError> {
    validate_file_path(path)?;

    let file = File::open(path)?;
    let mut records = Vec::new();

    if path.ends_with(".json") {
        records = serde_json::from_reader(file)?;
    } else {
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        for result in rdr.deserialize() {
            records.push(result?);
        }
    }

    Ok(records)
}

fn main() -> Result<(), AktenError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Validate { path } => {
            println!("🔍 Validating {} (medical mode: {})", path, cli.medical_mode);
            if !cli.dry_run {
                validate::run_validation(path, cli.medical_mode)?;
            }
        }

        Commands::Summarize { path } => {
            let records = load_records(path)?;
            summarize::summarize_data(&records, cli.medical_mode)?;
        }

        Commands::MergeFiles { output } => {
            println!("🔗 Merging files...");
            let sources = vec![
                "mock_data/patients_sample.csv",
                "mock_data/another_sample.csv",
            ];
            if cli.dry_run {
                println!("🧪 Dry-run: would merge files to '{}'.", output);
            } else {
                merge::merge_files(&sources, output, cli.medical_mode)?;
            }
        }

        Commands::Export { format, output } => {
            let records = load_records("mock_data/merged_output.csv")?;
            if cli.dry_run {
                println!("🧪 Dry-run: would export data to '{}'.", output);
            } else {
                export::export_data(&records, format, output, cli.medical_mode)?;
            }
        }

        Commands::ExportAi { output } => {
            let records = load_records("mock_data/merged_output.csv")?;
            if cli.dry_run {
                println!("🧪 Dry-run: would export AI data to '{}'.", output);
            } else {
                export::export_ai_data(&records, output)?;
            }
        }

        Commands::PredictRisk { path } => {
            let records = load_records(path)?;
            if cli.medical_mode {
                println!("⚕️ Medical risk prediction mode");
            }
            risk::predict_risks(&records)?;
        }

        Commands::ExportRiskJson { path, output } => {
            let records = load_records(path)?;
            if cli.dry_run {
                println!("🧪 Dry-run: would export predicted risk JSON to '{}'.", output);
            } else {
                risk::export_risks_as_json(&records, output)?;
            }
        }
    }

    Ok(())
}

mod validate;
mod summarize;
mod merge;
mod export;
mod risk;
mod config;
mod fhir;

use std::{path::Path, time::Instant};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, instrument, error};
use crate::config::ThresholdConfig;

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
    #[error("Unsupported file format (must be .csv, .json, or .fhir)")]
    UnsupportedFormat,
    #[error("Config load error: {0}")]
    ConfigError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Risk analysis error: {0}")]
    RiskError(String),
}

/// Patient health record structure
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[command(name = "AktenAkrobat", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable medical-specific processing
    #[arg(long)]
    medical_mode: bool,

    /// Perform dry run without file changes
    #[arg(long)]
    dry_run: bool,

    /// Config file path [default: config.toml]
    #[arg(long)]
    config: Option<String>,

    /// Enable verbose diagnostics
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate patient records
    Validate {
        #[arg(help = "Input file path")]
        path: String,
    },
    /// Generate summary statistics
    Summarize {
        #[arg(help = "Input file path")]
        path: String,
    },
    /// Merge record files
    MergeFiles {
        #[arg(help = "Output file path")]
        output: String,
        #[arg(help = "Input file paths")]
        inputs: Vec<String>,
    },
    /// Export records
    Export {
        #[arg(help = "Output format (csv|json)")]
        format: String,
        #[arg(help = "Output file path")]
        output: String,
    },
    /// Export AI-ready data
    ExportAi {
        #[arg(help = "Output file path")]
        output: String,
    },
    /// Predict health risks
    PredictRisk {
        #[arg(help = "Input file path")]
        path: String,
    },
    /// Export risk predictions
    ExportRiskJson {
        #[arg(help = "Input file path")]
        path: String,
        #[arg(help = "Output file path")]
        output: String,
    },
}

#[instrument]
fn main() -> Result<(), AktenError> {
    // Initialize logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config_path = cli.config.as_deref().unwrap_or("config.toml");
    let config = ThresholdConfig::load(config_path)
        .map_err(|e| AktenError::ConfigError(e.to_string()))?;

    info!(?config, "Loaded configuration");

    match &cli.command {
        Commands::Validate { path } => handle_validate(path, &cli, &config),
        Commands::Summarize { path } => handle_summarize(path, &cli),
        Commands::MergeFiles { output, inputs } => handle_merge(output, inputs, &cli),
        Commands::Export { format, output } => handle_export(format, output, &cli),
        Commands::ExportAi { output } => handle_export_ai(output, &cli),
        Commands::PredictRisk { path } => handle_predict_risk(path, &cli, &config),
        Commands::ExportRiskJson { path, output } => handle_export_risk(path, output, &cli, &config),
    }
}

// Command handlers
fn handle_validate(path: &str, cli: &Cli, config: &ThresholdConfig) -> Result<(), AktenError> {
    info!(path, "Validating records");
    if cli.dry_run {
        info!("Dry run - would validate {}", path);
        return Ok(());
    }
    validate::run_validation(path, cli.medical_mode, config)?;
    Ok(())
}

fn handle_summarize(path: &str, cli: &Cli) -> Result<(), AktenError> {
    let timer = Instant::now();
    let records = load_records(path)?;
    summarize::summarize_data(&records, cli.medical_mode)?;
    info!("Summary completed in {:?}", timer.elapsed());
    Ok(())
}

fn handle_merge(output: &str, inputs: &[String], cli: &Cli) -> Result<(), AktenError> {
    info!(?inputs, output, "Merging files");
    if cli.dry_run {
        info!("Dry run - would merge to {}", output);
        return Ok(());
    }
    let input_refs: Vec<&str> = inputs.iter().map(|s| s.as_str()).collect();
    merge::merge_files(&input_refs, output, cli.medical_mode)
}

fn handle_export(format: &str, output: &str, cli: &Cli) -> Result<(), AktenError> {
    let records = load_records("mock_data/merged_output.csv")?;
    if cli.dry_run {
        info!("Dry run - would export to {}", output);
        return Ok(());
    }
    export::export_data(&records, format, output, cli.medical_mode)
}

fn handle_export_ai(output: &str, cli: &Cli) -> Result<(), AktenError> {
    let records = load_records("mock_data/merged_output.csv")?;
    if cli.dry_run {
        info!("Dry run - would export AI data to {}", output);
        return Ok(());
    }
    export::export_ai_data(&records, output)
}

fn handle_predict_risk(path: &str, cli: &Cli, config: &ThresholdConfig) -> Result<(), AktenError> {
    let records = load_records(path)?;
    if cli.medical_mode {
        info!("Running in medical mode");
    }
    risk::predict_risks(&records, config)
}

fn handle_export_risk(path: &str, output: &str, cli: &Cli, config: &ThresholdConfig) -> Result<(), AktenError> {
    let records = load_records(path)?;
    if cli.dry_run {
        info!("Dry run - would export risks to {}", output);
        return Ok(());
    }
    risk::export_risks_as_json(&records, config, output)
}

// Core utilities
fn validate_path(path: &str) -> Result<(), AktenError> {
    if !Path::new(path).exists() {
        return Err(AktenError::InvalidPath(path.into()));
    }
    if !path.ends_with(".csv") && !path.ends_with(".json") && !path.ends_with(".fhir") {
        return Err(AktenError::UnsupportedFormat);
    }
    Ok(())
}

fn load_records(path: &str) -> Result<Vec<PatientRecord>, AktenError> {
    validate_path(path)?;
    info!(path, "Loading records");

    let file = std::fs::File::open(path)?;
    match path.rsplit('.').next() {
        Some("json") => serde_json::from_reader(file).map_err(Into::into),
        Some("fhir") => fhir::convert_fhir_to_records(file),
        Some("csv") => {
            csv::Reader::from_reader(file)
                .deserialize()
                .collect::<Result<_, _>>()
                .map_err(Into::into)
        }
        _ => Err(AktenError::UnsupportedFormat),
    }
}
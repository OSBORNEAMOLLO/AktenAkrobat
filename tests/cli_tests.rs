use std::process::Command;
use std::fs;
use std::path::Path;

#[test]
fn test_summarize_help() {
    let output = Command::new("target/debug/aktenakrobat")
        .arg("summarize")
        .arg("--help")
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Summarize health data"));
}

#[test]
fn test_validate_sample() {
    let output = Command::new("target/debug/aktenakrobat")
        .args(&["validate", "--path", "mock_data/patients_sample.csv"])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Validation complete"));
}

#[test]
fn test_merge_files_creates_output() {
    let output_file = "mock_data/merged_output.csv";
    let _ = fs::remove_file(output_file); // clean up if it exists

    let output = Command::new("target/debug/aktenakrobat")
        .arg("merge-files")
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    assert!(Path::new(output_file).exists());
}

#[test]
fn test_export_json() {
    let output_path = "export.json";
    let _ = fs::remove_file(output_path); // clean up if it exists

    // Merge first
    let _ = Command::new("target/debug/aktenakrobat")
        .arg("merge-files")
        .output()
        .expect("Failed to merge files");

    let output = Command::new("target/debug/aktenakrobat")
        .args(&["export", "--format", "json", "--output", output_path])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    assert!(Path::new(output_path).exists());
}

#[test]
fn test_export_csv() {
    let output_path = "export.csv";
    let _ = fs::remove_file(output_path); // clean up if it exists

    // Merge first
    let _ = Command::new("target/debug/aktenakrobat")
        .arg("merge-files")
        .output()
        .expect("Failed to merge files");

    let output = Command::new("target/debug/aktenakrobat")
        .args(&["export", "--format", "csv", "--output", output_path])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());
    assert!(Path::new(output_path).exists());
}

#[test]
fn test_predict_risk_command_runs() {
    // Merge first
    let _ = Command::new("target/debug/aktenakrobat")
        .arg("merge-files")
        .output()
        .expect("Failed to merge files");

    let output = Command::new("target/debug/aktenakrobat")
        .arg("predict-risk")
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Predicting risk") || stdout.contains("Risk Summary"),
        "Expected risk prediction output"
    );
}

use std::process::Command;

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

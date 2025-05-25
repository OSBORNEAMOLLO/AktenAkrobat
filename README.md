## 📦 Build Status

![Build](https://github.com/OSBORNEAMOLLO/AktenAkrobat/actions/workflows/main.yml/badge.svg)

#  MedIntegrator – Health Data CLI Toolkit
![Build](https://github.com/OSBORNEAMOLLO/AktenAkrobat/actions/workflows/main.yml/badge.svg)


**MedIntegrator** is a lightweight, Rust-powered command-line application for healthcare data management. It helps you **load, validate, merge, summarize, predict risks**, and export patient records — with special attention to **AI readiness** and **secure local use**.

---

##  Overview

Healthcare data often comes in different formats — spreadsheet exports, device logs, or clinical software dumps — making it difficult to consolidate and analyze.  
**MedIntegrator** offers a local-first, CLI-based tool that brings **structure and clarity** to this chaos, preparing data for analysis and machine learning — **without relying on cloud infrastructure**.

---

##  Key Features

- **File Import** – Load CSV or JSON files containing vitals, lab results, or logs  
- **Data Validation** – Detect errors, missing fields, and duplicates  
- **Summarization** – Compute health statistics like average BP, HR, and trends  
- **Merging Engine** – Combine multiple datasets from different time points  
- **AI-Ready Export** – Output structured JSON for ML model training  
- **Privacy-First Design** – All processing is local; no internet or cloud dependency

---

## 🦀 Why Rust?

MedIntegrator is written entirely in [**Rust**](https://www.rust-lang.org/) to achieve:

-  **Memory safety** (no segmentation faults)  
-  **High performance** like C/C++  
-  **Easy deployment**: statically linked binaries work out-of-the-box  
-  **Cross-platform builds** via `cross` and `musl`  
-  **Excellent CLI design** using [`clap`](https://crates.io/crates/clap)  
-  **Data parsing** via [`serde`, `serde_json`, and `csv`]  

---

## 📁 CLI Usage Examples

```bash
# Validate a health record file
aktenakrobat validate --path mock_data/patients_sample.csv

# Summarize patient data
aktenakrobat summarize --path mock_data/patients_sample.csv

# Merge predefined files
aktenakrobat merge-files

# Export merged data
aktenakrobat export --format csv --output export.csv

# Export normalized data for AI
aktenakrobat export-ai --output ai_data.json
```

---

##  Tech Stack & Libraries

- [`clap`] – Argument parsing and CLI commands  
- [`serde`] – Serialization of health records  
- [`csv`] – High-performance CSV parsing and writing  
- [`serde_json`] – Output structured JSON for AI-ready data  

---

## 📦 Cross-Platform Binary Support

Thanks to GitHub Actions + `cross`, the release pipeline compiles **statically linked** binaries for:

- `x86_64-unknown-linux-musl` – Standard Linux servers and desktops  
- `aarch64-unknown-linux-musl` – ARM-based devices (e.g., Raspberry Pi, edge computing nodes)

These binaries **do not require any dependencies** on the host OS.

---

## Future Goals

- Plug in a real ML model for risk prediction  
- Dockerized deployment for hospitals  
- Integration with electronic health record (EHR) APIs or FHIR  
- Mobile/GUI companion apps powered by the same backend

---

##  Innovation Highlight
MedIntegrator is not just a typical data processing CLI—it is uniquely crafted to address real healthcare data challenges:

-  **Rust-powered CLI with performance in mind**: Built using Rust for memory safety, speed, and zero-cost abstractions. It compiles to a single binary with no runtime dependencies.
-  **Cross-Platform AI Readiness**: Unlike typical tools, this app outputs normalized JSON data that is plug-and-play with Python ML pipelines.
-  **Statically Linked Linux Builds**: Thanks to `cross` and GitHub Actions, the tool builds clean binaries for `x86_64` and `aarch64` (e.g., Raspberry Pi), making it hospital-server and edge-device ready.
-  **Local-First Privacy**: Everything runs locally—no cloud dependency, protecting sensitive patient health data.
- **Academic & Business-Ready**: Combines software engineering best practices (CI, modular design, structured tests) with a scalable healthcare vision.

## 📦 Build Status

![Build](https://github.com/OSBORNEAMOLLO/AktenAkrobat/actions/workflows/main.yml/badge.svg)

# 🧠 AktenAkrobat – Health Data CLI Toolkit

AktenAkrobat is a lightweight, Rust-powered command-line application for healthcare data management. It helps you **load, validate, merge, summarize, predict risks**, and export patient records — with special attention to **AI readiness** and **secure local use**.

---

## 📌 Overview

Healthcare data often comes in different formats — spreadsheet exports, device logs, or clinical software dumps — making it difficult to consolidate and analyze.  
**AktenAkrobat** offers a local-first, CLI-based tool that brings **structure and clarity** to this chaos, preparing data for analysis and machine learning — **without relying on cloud infrastructure**.

---

## 🚀 Key Features

- Load CSV or JSON files containing vitals, lab results, or logs  
- Detect errors, missing fields, and duplicates  
- Compute health statistics like average BP, HR, and trends  
- Combine multiple datasets from different time points  
- Output structured JSON for ML model training  
- All processing is local; no internet or cloud dependency

---

## 🦀 Why Rust?

AktenAkrobat is written entirely in [**Rust**](https://www.rust-lang.org/) to achieve:

- Memory safety (no segmentation faults)  
- High performance like C/C++  
- Easy deployment via statically linked binaries  
- Cross-platform builds using `cross` + `musl`  
- Excellent CLI with [`clap`](https://crates.io/crates/clap)  
- Data parsing via [`serde`, `serde_json`, `csv`]  

---

## 📁 CLI Usage Examples

```bash
aktenakrobat validate --path mock_data/patients_sample.csv
aktenakrobat summarize --path mock_data/patients_sample.csv
aktenakrobat merge-files
aktenakrobat export --format csv --output export.csv
aktenakrobat export-ai --output ai_data.json
```

---

## 🧰 Tech Stack & Libraries

- [`clap`] – CLI argument parser  
- [`serde`] – Serialization  
- [`csv`] – Fast CSV handling  
- [`serde_json`] – Structured AI-ready JSON output  

---

## 🐧 Linux Platform Compatibility

Uses **GitHub Actions** + `cross` to build statically linked binaries for:

- `x86_64-unknown-linux-musl` (standard Linux servers/desktops)  
- `aarch64-unknown-linux-musl` (ARM devices like Raspberry Pi)  

✅ These binaries **do not require Rust or any dependencies** — just download and execute.

---

## 💡 Personal Automation

- Automates repetitive tasks like validating, summarizing, merging, exporting, and risk-checking records  
- Designed for researchers, caregivers, or developers managing health data without cloud reliance  

---

## 🧠 Innovation Layer

- **AI-Ready** – JSON structured for ML model input  
- **Local-First** – No network needed; runs offline securely  
- **CLI Simplicity** – Rust-powered, fast & minimal interface  
- **Future-Proof** – Extendable to ML, Docker, or GUI apps  

---

## 🎯 Future Goals

- Plug in a real ML model for risk prediction  
- Dockerized deployment for hospitals  
- Integration with electronic health record (EHR) APIs or FHIR  
- Mobile/GUI companion apps powered by the same backend

---
## 🧑‍🎓 Why I Took This Path

1. **Practical Application of My Studies**  
   I wanted a meaningful, reusable tool combining Health Informatics, Rust, and Business Game insights.  

2. **Vision Beyond the Assignment**  
   Inspired by *Termin Zauberer*, I aimed for real-world problem-solving: privacy, automation, AI-readiness, and deployment.

---

## ✅ Outcome

This project fulfills the personal automation task **and** lays a foundation for **real innovation** and healthcare tech applications.

---
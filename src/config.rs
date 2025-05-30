use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

/// Error type for configuration loading and validation
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Invalid threshold value: {0}")]
    InvalidThreshold(String),
}

/// Main configuration structure containing all thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    pub thresholds: Thresholds,
}

/// Collection of all medical thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thresholds {
    #[serde(rename = "critical_hr")]
    pub heart_rate: CriticalHr,
    #[serde(rename = "hypertensive_crisis")]
    pub blood_pressure: HypertensiveCrisis,
    pub hypothermia: f32,
    pub fever: f32,
    pub hypoglycemia: f32,
    pub hyperglycemia: f32,
}

/// Heart rate thresholds (bpm)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalHr {
    pub min: u32,
    pub max: u32,
}

/// Blood pressure thresholds (mmHg)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypertensiveCrisis {
    pub systolic: u32,
    pub diastolic: u32,
}

impl ThresholdConfig {
    /// Loads and validates configuration from a TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config: ThresholdConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Validates all threshold values make medical sense
    fn validate(&self) -> Result<(), ConfigError> {
        // Validate heart rate thresholds
        if self.thresholds.heart_rate.min >= self.thresholds.heart_rate.max {
            return Err(ConfigError::InvalidThreshold(
                "Heart rate min must be less than max".to_string(),
            ));
        }

        // Validate blood pressure thresholds
        if self.thresholds.blood_pressure.systolic <= 0
            || self.thresholds.blood_pressure.diastolic <= 0
        {
            return Err(ConfigError::InvalidThreshold(
                "Blood pressure values must be positive".to_string(),
            ));
        }

        // Validate temperature thresholds
        if self.thresholds.hypothermia >= self.thresholds.fever {
            return Err(ConfigError::InvalidThreshold(
                "Hypothermia threshold must be lower than fever threshold".to_string(),
            ));
        }

        // Validate glucose thresholds
        if self.thresholds.hypoglycemia >= self.thresholds.hyperglycemia {
            return Err(ConfigError::InvalidThreshold(
                "Hypoglycemia threshold must be lower than hyperglycemia threshold".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config = ThresholdConfig {
            thresholds: Thresholds {
                heart_rate: CriticalHr { min: 60, max: 100 },
                blood_pressure: HypertensiveCrisis {
                    systolic: 180,
                    diastolic: 120,
                },
                hypothermia: 35.0,
                fever: 38.0,
                hypoglycemia: 3.9,
                hyperglycemia: 7.0,
            },
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config() {
        let invalid_config = ThresholdConfig {
            thresholds: Thresholds {
                heart_rate: CriticalHr { min: 100, max: 60 }, // Invalid
                blood_pressure: HypertensiveCrisis {
                    systolic: 180,
                    diastolic: 120,
                },
                hypothermia: 35.0,
                fever: 38.0,
                hypoglycemia: 7.0, // Invalid (higher than hyperglycemia)
                hyperglycemia: 3.9,
            },
        };

        assert!(invalid_config.validate().is_err());
    }
}
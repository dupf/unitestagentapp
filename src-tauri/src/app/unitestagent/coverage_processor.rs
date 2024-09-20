use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use xml::reader::{EventReader, XmlEvent};
use csv::Reader;
use regex::Regex;
use std::result::Result;

pub struct CoverageProcessor {
    file_path: String,
    src_file_path: String,
    coverage_type: CoverageType,
    use_report_coverage_feature_flag: bool,
}

#[derive(PartialEq)]
pub enum CoverageType {
    Cobertura,
    Lcov,
    Jacoco,
}

impl CoverageProcessor {
    pub fn new(
        file_path: String,
        src_file_path: String,
        coverage_type: CoverageType,
        use_report_coverage_feature_flag: bool,
    ) -> Self {
        Self {
            file_path,
            src_file_path,
            coverage_type,
            use_report_coverage_feature_flag,
        }
    }
    pub fn verify_report_update(&self, time_of_test_command: u128) -> Result<(), String> {
        let path = Path::new(&self.file_path);

        if !path.exists() {
            return Err(format!("Fatal: Coverage report \"{}\" was not generated.", self.file_path));
        }

        let metadata = std::fs::metadata(path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let file_mod_time = metadata.modified().map_err(|e| format!("Failed to get file modification time: {}", e))?;
        
        let file_mod_time_ms = file_mod_time
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Failed to calculate duration: {}", e))?
            .as_millis();

        if file_mod_time_ms <= time_of_test_command {
            return Err(format!(
                "Fatal: The coverage report file was not updated after the test command. \
                file_mod_time_ms: {}, time_of_test_command: {}. {}",
                file_mod_time_ms,
                time_of_test_command,
                file_mod_time_ms > time_of_test_command
            ));
        }

        Ok(())
    }
 
    pub fn process_coverage_report(&self, time_of_test_command: u128) -> Result<(Vec<u32>, Vec<u32>, f64), String> {
        self.verify_report_update(time_of_test_command)?;
        self.parse_coverage_report();
    }

    fn parse_coverage_report(&self) -> Result<(Vec<u32>, Vec<u32>, f64)> {
        match self.coverage_type {
            CoverageType::Cobertura => {
                if self.use_report_coverage_feature_flag {
                    self.parse_coverage_report_cobertura(None)
                } else {
                    let filename = Path::new(&self.src_file_path)
                        .file_name()
                        .context("Invalid src_file_path")?
                        .to_str()
                        .context("Invalid UTF-8 in filename")?
                        .to_string();
                    self.parse_coverage_report_cobertura(Some(filename))
                }
            }
            CoverageType::Lcov => self.parse_coverage_report_lcov(),
            CoverageType::Jacoco => self.parse_coverage_report_jacoco(),
        }
    }

    fn parse_coverage_report_cobertura(&self, filename: Option<String>) -> Result<(Vec<u32>, Vec<u32>, f64)> {
        // Implementation for parsing Cobertura XML
        // This would involve using the xml crate to parse the XML file
        // and extract the necessary information
        unimplemented!("parse_coverage_report_cobertura not implemented")
    }

    fn parse_coverage_report_lcov(&self) -> Result<(Vec<u32>, Vec<u32>, f64)> {
        let mut lines_covered = Vec::new();
        let mut lines_missed = Vec::new();
        let filename = Path::new(&self.src_file_path)
            .file_name()
            .context("Invalid src_file_path")?
            .to_str()
            .context("Invalid UTF-8 in filename")?;

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let mut in_target_file = false;
        for line in reader.lines() {
            let line = line?;
            if line.starts_with("SF:") && line.ends_with(filename) {
                in_target_file = true;
                continue;
            }
            if in_target_file {
                if line.starts_with("DA:") {
                    let parts: Vec<&str> = line.trim_start_matches("DA:").split(',').collect();
                    if parts.len() == 2 {
                        let line_number: u32 = parts[0].parse()?;
                        let hits: u32 = parts[1].parse()?;
                        if hits > 0 {
                            lines_covered.push(line_number);
                        } else {
                            lines_missed.push(line_number);
                        }
                    }
                } else if line.starts_with("end_of_record") {
                    break;
                }
            }
        }

        let total_lines = lines_covered.len() + lines_missed.len();
        let coverage_percentage = if total_lines > 0 {
            lines_covered.len() as f64 / total_lines as f64
        } else {
            0.0
        };

        Ok((lines_covered, lines_missed, coverage_percentage))
    }

    fn parse_coverage_report_jacoco(&self) -> Result<(Vec<u32>, Vec<u32>, f64)> {
        let (package_name, class_name) = self.extract_package_and_class_java()?;
        let (missed, covered) = self.parse_missed_covered_lines_jacoco(&package_name, &class_name)?;

        let total_lines = missed + covered;
        let coverage_percentage = if total_lines > 0 {
            covered as f64 / total_lines as f64
        } else {
            0.0
        };

        Ok((Vec::new(), Vec::new(), coverage_percentage))
    }

    fn parse_missed_covered_lines_jacoco(&self, package_name: &str, class_name: &str) -> Result<(u32, u32)> {
        let mut reader = csv::Reader::from_path(&self.file_path)?;
        
        for result in reader.records() {
            let record = result?;
            if record.get(0) == Some(package_name) && record.get(1) == Some(class_name) {
                let missed: u32 = record.get(4).context("Missing LINE_MISSED column")?.parse()?;
                let covered: u32 = record.get(5).context("Missing LINE_COVERED column")?.parse()?;
                return Ok((missed, covered));
            }
        }

        anyhow::bail!("Package and class not found in JaCoCo report")
    }

    fn extract_package_and_class_java(&self) -> Result<(String, String)> {
        let file = File::open(&self.src_file_path)?;
        let reader = BufReader::new(file);

        let package_pattern = Regex::new(r"^\s*package\s+([\w\.]+)\s*;.*$")?;
        let class_pattern = Regex::new(r"^\s*public\s+class\s+(\w+).*")?;

        let mut package_name = String::new();
        let mut class_name = String::new();

        for line in reader.lines() {
            let line = line?;
            if package_name.is_empty() {
                if let Some(captures) = package_pattern.captures(&line) {
                    package_name = captures[1].to_string();
                }
            }
            if class_name.is_empty() {
                if let Some(captures) = class_pattern.captures(&line) {
                    class_name = captures[1].to_string();
                }
            }
            if !package_name.is_empty() && !class_name.is_empty() {
                break;
            }
        }

        if package_name.is_empty() || class_name.is_empty() {
            anyhow::bail!("Failed to extract package and class names");
        }

        Ok((package_name, class_name))
    }
}
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use log::{error, info};

/// 覆盖率报告类型
#[derive(Debug, Clone)]
pub enum CoverageType {
    Cobertura,
    Lcov,
    Jacoco,
}

impl CoverageType {
    /// 从字符串解析覆盖率类型
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cobertura" | "coverage" => CoverageType::Cobertura,
            "lcov" => CoverageType::Lcov,
            "jacoco" => CoverageType::Jacoco,
            _ => CoverageType::Lcov,
        }
    }
}

/// 覆盖率报告处理器
pub struct CoverageProcessor {
    file_path: String,
    src_file_path: String,
    coverage_type: CoverageType,
    use_report_coverage_feature_flag: bool,
}

/// 覆盖率解析结果
#[derive(Debug, Clone)]
pub struct CoverageResult {
    pub lines_covered: Vec<i32>,
    pub lines_missed: Vec<i32>,
    pub coverage_percentage: f64,
}

impl CoverageResult {
    /// 返回空的覆盖率结果
    pub fn empty() -> Self {
        CoverageResult {
            lines_covered: vec![],
            lines_missed: vec![],
            coverage_percentage: 0.0,
        }
    }

    /// 生成可读的覆盖率摘要文本，用于注入提示词
    pub fn to_report_text(&self) -> String {
        let total = self.lines_covered.len() + self.lines_missed.len();
        if total == 0 {
            return "No coverage data available.".to_string();
        }

        let mut report = format!(
            "Coverage: {:.1}% ({}/{} lines covered)\n",
            self.coverage_percentage * 100.0,
            self.lines_covered.len(),
            total
        );

        if !self.lines_missed.is_empty() {
            let missed_str: Vec<String> = self.lines_missed.iter().map(|l| l.to_string()).collect();
            report.push_str(&format!(
                "Uncovered lines: {}\n",
                missed_str.join(", ")
            ));
        }

        report
    }
}

impl CoverageProcessor {
    pub fn new(
        file_path: String,
        src_file_path: String,
        coverage_type: CoverageType,
        use_report_coverage_feature_flag: bool,
    ) -> Self {
        CoverageProcessor {
            file_path,
            src_file_path,
            coverage_type,
            use_report_coverage_feature_flag,
        }
    }

    /// 校验覆盖率报告是否在测试命令之后被更新
    pub fn verify_report_update(&self, time_of_test_command: i64) -> io::Result<()> {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Fatal: Coverage report '{}' was not generated.",
                    self.file_path
                ),
            ));
        }

        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let file_mod_time_ms = modified
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        if file_mod_time_ms <= time_of_test_command {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Fatal: Coverage report was not updated after test command. \
                    file_mod_time_ms: {}, time_of_test_command: {}",
                    file_mod_time_ms, time_of_test_command
                ),
            ));
        }

        Ok(())
    }

    /// 处理覆盖率报告（含时间验证）
    pub fn process_coverage_report(
        &self,
        time_of_test_command: i64,
    ) -> io::Result<CoverageResult> {
        self.verify_report_update(time_of_test_command)?;
        self.parse_coverage_report()
    }

    /// 直接解析覆盖率报告（不做时间验证，用于初始读取）
    pub fn parse_coverage_report_direct(&self) -> io::Result<CoverageResult> {
        if !Path::new(&self.file_path).exists() {
            return Ok(CoverageResult::empty());
        }
        self.parse_coverage_report()
    }

    fn parse_coverage_report(&self) -> io::Result<CoverageResult> {
        match self.coverage_type {
            CoverageType::Cobertura => {
                if self.use_report_coverage_feature_flag {
                    self.parse_coverage_report_cobertura(None)
                } else {
                    let filename = Path::new(&self.src_file_path)
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or("")
                        .to_string();
                    self.parse_coverage_report_cobertura(Some(filename))
                }
            }
            CoverageType::Lcov => self.parse_coverage_report_lcov(),
            CoverageType::Jacoco => self.parse_coverage_report_jacoco(),
        }
    }

    // =========================================================================
    // Cobertura XML Parser（基于 regex 解析，无需 quick_xml 依赖）
    // =========================================================================
    // Cobertura XML 结构:
    //   <class name="..." filename="src/foo.py" line-rate="0.8">
    //     <lines>
    //       <line number="1" hits="1"/>
    //       <line number="2" hits="0"/>
    //     </lines>
    //   </class>
    // =========================================================================
    fn parse_coverage_report_cobertura(
        &self,
        filename: Option<String>,
    ) -> io::Result<CoverageResult> {
        let content = fs::read_to_string(&self.file_path)?;
        let mut lines_covered: Vec<i32> = Vec::new();
        let mut lines_missed: Vec<i32> = Vec::new();

        let target_filename = filename.unwrap_or_else(|| {
            Path::new(&self.src_file_path)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("")
                .to_string()
        });

        let class_re =
            Regex::new(r#"<class[^>]*filename="([^"]*)"[^>]*>"#).unwrap();
        let line_re =
            Regex::new(r#"<line[^>]*number="(\d+)"[^>]*hits="(\d+)"[^>]*/?\s*>"#).unwrap();
        let class_end_re = Regex::new(r"</class>").unwrap();

        let mut in_target_class = false;

        for line in content.lines() {
            let line = line.trim();

            if let Some(caps) = class_re.captures(line) {
                let class_filename = &caps[1];
                in_target_class = if target_filename.is_empty() {
                    true
                } else {
                    class_filename.contains(&target_filename)
                        || target_filename.contains(class_filename)
                        || class_filename.ends_with(&target_filename)
                };
                continue;
            }

            if class_end_re.is_match(line) {
                if in_target_class && !target_filename.is_empty() {
                    break;
                }
                in_target_class = false;
                continue;
            }

            if in_target_class || target_filename.is_empty() {
                if let Some(caps) = line_re.captures(line) {
                    let line_number: i32 = caps[1].parse().unwrap_or(0);
                    let hits: i32 = caps[2].parse().unwrap_or(0);

                    if hits > 0 {
                        if !lines_covered.contains(&line_number) {
                            lines_covered.push(line_number);
                        }
                    } else {
                        if !lines_missed.contains(&line_number) {
                            lines_missed.push(line_number);
                        }
                    }
                }
            }
        }

        let total_lines = lines_covered.len() + lines_missed.len();
        let coverage_percentage = if total_lines > 0 {
            lines_covered.len() as f64 / total_lines as f64
        } else {
            0.0
        };

        info!(
            "Cobertura coverage: {:.1}% ({}/{} lines)",
            coverage_percentage * 100.0,
            lines_covered.len(),
            total_lines
        );

        Ok(CoverageResult {
            lines_covered,
            lines_missed,
            coverage_percentage,
        })
    }

    // =========================================================================
    // LCOV Parser
    // =========================================================================
    fn parse_coverage_report_lcov(&self) -> io::Result<CoverageResult> {
        let mut lines_covered = Vec::new();
        let mut lines_missed = Vec::new();

        let filename = Path::new(&self.src_file_path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let mut in_target_file = false;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim().to_string();

            if line.starts_with("SF:") && line.ends_with(filename) {
                in_target_file = true;
                continue;
            }

            if in_target_file {
                if line.starts_with("DA:") {
                    let data = line.replace("DA:", "");
                    let parts: Vec<&str> = data.split(',').collect();
                    if parts.len() >= 2 {
                        let line_number = parts[0].trim().parse::<i32>().unwrap_or(0);
                        let hits = parts[1].trim().parse::<i32>().unwrap_or(0);

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

        info!(
            "LCOV coverage: {:.1}% ({}/{} lines)",
            coverage_percentage * 100.0,
            lines_covered.len(),
            total_lines
        );

        Ok(CoverageResult {
            lines_covered,
            lines_missed,
            coverage_percentage,
        })
    }

    // =========================================================================
    // Jacoco XML Parser（基于 regex 解析）
    // =========================================================================
    // Jacoco XML 结构:
    //   <sourcefile name="MyClass.java">
    //     <line nr="1" mi="0" ci="3" mb="0" cb="0"/>
    //   </sourcefile>
    // 其中: mi=missed instructions, ci=covered instructions
    //       mb=missed branches, cb=covered branches
    // ci > 0 表示该行已覆盖; mi > 0 && ci == 0 表示未覆盖
    // =========================================================================
    fn parse_coverage_report_jacoco(&self) -> io::Result<CoverageResult> {
        let content = fs::read_to_string(&self.file_path)?;
        let mut lines_covered: Vec<i32> = Vec::new();
        let mut lines_missed: Vec<i32> = Vec::new();

        let target_filename = Path::new(&self.src_file_path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_string();

        let sourcefile_re =
            Regex::new(r#"<sourcefile[^>]*name="([^"]*)"[^>]*>"#).unwrap();
        let line_re = Regex::new(
            r#"<line[^>]*nr="(\d+)"[^>]*mi="(\d+)"[^>]*ci="(\d+)"[^>]*/?\s*>"#,
        )
        .unwrap();
        let sourcefile_end_re = Regex::new(r"</sourcefile>").unwrap();

        let mut in_target_source = false;

        for line in content.lines() {
            let line = line.trim();

            if let Some(caps) = sourcefile_re.captures(line) {
                let source_name = &caps[1];
                in_target_source = source_name == target_filename
                    || source_name.contains(&target_filename)
                    || target_filename.contains(source_name);
                continue;
            }

            if sourcefile_end_re.is_match(line) {
                if in_target_source {
                    break;
                }
                in_target_source = false;
                continue;
            }

            if in_target_source {
                if let Some(caps) = line_re.captures(line) {
                    let line_number: i32 = caps[1].parse().unwrap_or(0);
                    let missed_instructions: i32 = caps[2].parse().unwrap_or(0);
                    let covered_instructions: i32 = caps[3].parse().unwrap_or(0);

                    if covered_instructions > 0 {
                        lines_covered.push(line_number);
                    } else if missed_instructions > 0 {
                        lines_missed.push(line_number);
                    }
                }
            }
        }

        let total_lines = lines_covered.len() + lines_missed.len();
        let coverage_percentage = if total_lines > 0 {
            lines_covered.len() as f64 / total_lines as f64
        } else {
            0.0
        };

        info!(
            "Jacoco coverage: {:.1}% ({}/{} lines)",
            coverage_percentage * 100.0,
            lines_covered.len(),
            total_lines
        );

        Ok(CoverageResult {
            lines_covered,
            lines_missed,
            coverage_percentage,
        })
    }
}

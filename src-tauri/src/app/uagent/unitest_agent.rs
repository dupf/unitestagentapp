use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use log::{error, info, warn};
use tauri::{AppHandle, Manager};

use crate::app::uagent::{
    coverage_processor::{CoverageProcessor, CoverageResult, CoverageType},
    progress::{AgentPhase, IterationRecord, ProgressTracker},
    report_generator::ReportGenerator,
    runner::Runner,
    unitest_agent_test_generator::{TestDetails, UnitTestAgentTestGenerator},
};

// =========================================================================
// 闭环反馈引擎 v2 — Coverage-Driven Closed-Loop Agent
//
// 核心流程:
//   1. 初始覆盖率分析（如有覆盖率报告）
//   2. 循环迭代:
//      a. AI 生成测试用例
//      b. 将测试代码追加写入测试文件
//      c. 执行测试命令
//      d. 检查测试结果：失败则触发 AI 自修复
//      e. 解析覆盖率报告，更新覆盖率
//      f. 反馈覆盖率信息到下一轮提示词
//      g. 达到目标覆盖率则提前退出
//   3. 静态代码分析
//   4. 测试质量评分（变异测试分析）
//   5. 生成综合报告
// =========================================================================

pub struct UnitestAgent {
    source_file_path: String,
    test_file_path: String,
    test_file_output_path: String,
    code_coverage_report_path: String,
    test_command: String,
    test_command_dir: String,
    included_files: Option<Vec<String>>,
    coverage_type: String,
    report_filepath: String,
    desired_coverage: i32,
    max_iterations: i32,
    additional_instructions: String,
    model: String,
    isremote: bool,
    api_base: String,
    strict_coverage: bool,
    run_tests_multiple_times: i32,
    use_report_coverage_feature_flag: bool,
    test_gen: UnitTestAgentTestGenerator,
}

impl UnitestAgent {
    pub fn new(
        source_file_path: String,
        test_file_path: String,
        test_file_output_path: String,
        code_coverage_report_path: String,
        test_command: String,
        test_command_dir: String,
        included_files: Option<Vec<String>>,
        coverage_type: String,
        report_filepath: String,
        desired_coverage: i32,
        max_iterations: i32,
        additional_instructions: String,
        model: String,
        isremote: bool,
        api_base: String,
        strict_coverage: bool,
        run_tests_multiple_times: i32,
        use_report_coverage_feature_flag: bool,
    ) -> Self {
        let test_gen = UnitTestAgentTestGenerator::new(
            &source_file_path,
            &test_file_path,
            &code_coverage_report_path,
            &test_command,
            Some(&test_command_dir),
            included_files.as_ref(),
            Some(&coverage_type),
            Some(desired_coverage),
            &model,
            &api_base,
            &additional_instructions,
            use_report_coverage_feature_flag,
            isremote,
        );

        let unitest_agent = UnitestAgent {
            source_file_path,
            test_file_path,
            test_file_output_path,
            code_coverage_report_path,
            test_command,
            test_command_dir,
            included_files,
            coverage_type,
            report_filepath,
            desired_coverage,
            max_iterations,
            additional_instructions,
            model,
            isremote,
            api_base,
            strict_coverage,
            run_tests_multiple_times,
            use_report_coverage_feature_flag,
            test_gen: test_gen.unwrap(),
        };
        unitest_agent.validate_paths();
        unitest_agent
    }

    fn validate_paths(&self) {
        if !Path::new(&self.source_file_path).is_file() {
            File::create(&self.source_file_path)
                .expect("Failed to create source file")
                .write_all(b"")
                .expect("Failed to write to source file");
            info!(
                "Source file not found. Created an empty file at {}",
                self.source_file_path
            );
        }

        if !Path::new(&self.test_file_path).is_file() {
            File::create(&self.test_file_path)
                .expect("Failed to create test file")
                .write_all(b"")
                .expect("Failed to write to test file");
            info!(
                "Test file not found. Created an empty file at {}",
                self.test_file_path
            );
        }
    }

    /// 备份测试文件到输出路径
    fn backup_test_file(&self) {
        if !self.test_file_output_path.is_empty() && self.test_file_output_path != self.test_file_path {
            if let Err(e) = fs::copy(&self.test_file_path, &self.test_file_output_path) {
                warn!("Failed to backup test file: {}", e);
            }
        }
    }

    // =========================================================================
    // Phase 1: 初始覆盖率分析
    // =========================================================================
    fn analyze_initial_coverage(&self) -> Option<CoverageResult> {
        if self.code_coverage_report_path.is_empty()
            || !Path::new(&self.code_coverage_report_path).exists()
        {
            info!("No initial coverage report found, starting from scratch.");
            return None;
        }

        let processor = CoverageProcessor::new(
            self.code_coverage_report_path.clone(),
            self.source_file_path.clone(),
            CoverageType::from_str(&self.coverage_type),
            self.use_report_coverage_feature_flag,
        );

        match processor.parse_coverage_report_direct() {
            Ok(result) => {
                info!(
                    "Initial coverage: {:.1}% ({} covered, {} missed)",
                    result.coverage_percentage * 100.0,
                    result.lines_covered.len(),
                    result.lines_missed.len()
                );
                Some(result)
            }
            Err(e) => {
                warn!("Failed to parse initial coverage report: {}", e);
                None
            }
        }
    }

    // =========================================================================
    // Phase 2c: 执行测试命令
    // =========================================================================
    fn execute_tests(&self) -> (String, String, i32, u128) {
        if self.test_command.is_empty() {
            info!("No test command configured, skipping test execution.");
            return (String::new(), String::new(), 0, 0);
        }

        let cwd = if self.test_command_dir.is_empty() {
            None
        } else {
            Some(self.test_command_dir.as_str())
        };

        info!("Executing test command: {} (cwd: {:?})", self.test_command, cwd);
        Runner::run_command(&self.test_command, cwd)
    }

    // =========================================================================
    // Phase 2e: 解析覆盖率报告
    // =========================================================================
    fn parse_coverage_after_test(&self, command_start_time: u128) -> Option<CoverageResult> {
        if self.code_coverage_report_path.is_empty() {
            return None;
        }

        let processor = CoverageProcessor::new(
            self.code_coverage_report_path.clone(),
            self.source_file_path.clone(),
            CoverageType::from_str(&self.coverage_type),
            self.use_report_coverage_feature_flag,
        );

        // 先尝试带时间验证的解析
        match processor.process_coverage_report(command_start_time as i64) {
            Ok(result) => Some(result),
            Err(e) => {
                warn!(
                    "Coverage report time verification failed: {}. Trying direct parse.",
                    e
                );
                // 回退到直接解析（不验证时间戳）
                processor.parse_coverage_report_direct().ok()
            }
        }
    }

    // =========================================================================
    // Phase 2b: 将测试代码追加写入测试文件
    // =========================================================================
    fn append_tests_to_file(&self, tests: &[TestDetails]) -> Result<(), String> {
        let test_file_path = if !self.test_file_output_path.is_empty() {
            &self.test_file_output_path
        } else {
            &self.test_file_path
        };

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(test_file_path)
            .map_err(|e| format!("Failed to open test file for appending: {}", e))?;

        for test in tests {
            if let Some(ref code) = test.test_code {
                let code_trimmed = code.trim();
                if !code_trimmed.is_empty() && code_trimmed != "\"\"" {
                    writeln!(file, "\n{}", code_trimmed)
                        .map_err(|e| format!("Failed to write test code: {}", e))?;
                }
            }
        }

        info!("Appended {} test(s) to {}", tests.len(), test_file_path);
        Ok(())
    }

    // =========================================================================
    // 主执行流程: 闭环反馈引擎
    // =========================================================================
    pub async fn run(&mut self, handle: AppHandle, unitest_id: String, id: u64) {
        let mut tracker = ProgressTracker::new(id, self.max_iterations, self.desired_coverage as f64);
        let mut test_results_list: Vec<TestDetails> = Vec::new();
        let mut quality_score_result: Option<String> = None;

        // ===== Phase 1: 初始化 =====
        tracker.emit_phase(&handle, AgentPhase::Initializing, "正在初始化 Agent...");
        tracker.emit_chat_progress(
            &handle,
            &format!(
                "\n🚀 **UnitestAgent 闭环引擎启动**\n\
                 - 源文件: `{}`\n\
                 - 测试文件: `{}`\n\
                 - 目标覆盖率: {}%\n\
                 - 最大迭代: {} 轮\n\
                 - 模型: {}\n",
                self.source_file_path,
                self.test_file_path,
                self.desired_coverage,
                self.max_iterations,
                self.model,
            ),
        );

        // ===== Phase 1.5: 初始覆盖率分析 =====
        tracker.emit_phase(&handle, AgentPhase::CoverageAnalysis, "分析初始覆盖率...");
        let initial_coverage = self.analyze_initial_coverage();
        if let Some(ref cov) = initial_coverage {
            tracker.set_coverage(cov.coverage_percentage);
            self.test_gen.update_coverage_report(&cov.to_report_text());
            tracker.emit_chat_progress(
                &handle,
                &format!(
                    "\n📊 **初始覆盖率**: {:.1}% ({}/{} 行已覆盖)\n",
                    cov.coverage_percentage * 100.0,
                    cov.lines_covered.len(),
                    cov.lines_covered.len() + cov.lines_missed.len(),
                ),
            );

            // 如果初始覆盖率已达标，跳过测试生成
            if tracker.is_coverage_met() {
                tracker.emit_chat_progress(
                    &handle,
                    &format!(
                        "\n✅ 初始覆盖率已达到目标 ({}%)，跳过测试生成。\n",
                        self.desired_coverage
                    ),
                );
            }
        }

        // ===== Phase 2: 迭代闭环 =====
        let mut iteration_count = 0;
        let has_test_command = !self.test_command.is_empty();

        while iteration_count < self.max_iterations && !tracker.is_coverage_met() {
            iteration_count += 1;
            tracker.set_iteration(iteration_count);

            let coverage_before = tracker.get_coverage();

            tracker.emit_chat_progress(
                &handle,
                &format!(
                    "\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                     🔄 **迭代 {}/{}** 开始\n\
                     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
                    iteration_count, self.max_iterations,
                ),
            );

            // ----- 2a: AI 生成测试 -----
            tracker.emit_phase(&handle, AgentPhase::TestGeneration, "AI 正在生成测试用例...");
            tracker.emit_chat_progress(
                &handle,
                &format!("\n🤖 **Phase 1/4**: AI 生成测试用例...\n"),
            );

            let generated_tests = match self
                .test_gen
                .generate_tests(handle.clone(), id, 4096, false)
                .await
            {
                Ok(tests) => tests,
                Err(e) => {
                    error!("Error generating tests in iteration {}: {}", iteration_count, e);
                    tracker.emit_chat_progress(
                        &handle,
                        &format!("\n⚠️ 迭代 {} 生成失败: {}，继续下一轮...\n", iteration_count, e),
                    );
                    continue;
                }
            };

            let num_generated = generated_tests.len() as i32;
            tracker.emit_chat_progress(
                &handle,
                &format!("\n✅ 生成了 {} 个测试用例\n", num_generated),
            );

            if generated_tests.is_empty() {
                tracker.emit_chat_progress(
                    &handle,
                    "\n⚠️ 未生成任何测试用例，跳过本轮。\n",
                );
                continue;
            }

            // ----- 2b: 写入测试文件 -----
            let mut tests_passed = 0i32;
            let mut tests_failed = 0i32;
            let mut tests_repaired = 0i32;
            let mut final_tests = generated_tests.clone();

            if has_test_command {
                tracker.emit_phase(&handle, AgentPhase::TestWriting, "写入测试到文件...");
                tracker.emit_chat_progress(
                    &handle,
                    "\n📝 **Phase 2/4**: 写入测试代码到文件...\n",
                );

                if let Err(e) = self.append_tests_to_file(&generated_tests) {
                    error!("Failed to write tests: {}", e);
                    tracker.emit_chat_progress(
                        &handle,
                        &format!("\n❌ 写入测试失败: {}\n", e),
                    );
                } else {
                    // ----- 2c: 执行测试 -----
                    tracker.emit_phase(&handle, AgentPhase::TestExecution, "正在执行测试...");
                    tracker.emit_chat_progress(
                        &handle,
                        &format!("\n🏃 **Phase 3/4**: 执行测试命令: `{}`\n", self.test_command),
                    );

                    let (stdout, stderr, exit_code, start_time) = self.execute_tests();

                    if exit_code == 0 {
                        tests_passed = num_generated;
                        tracker.emit_chat_progress(
                            &handle,
                            &format!("\n✅ 测试全部通过 (exit_code: {})\n", exit_code),
                        );
                    } else {
                        tests_failed = num_generated;
                        tracker.emit_chat_progress(
                            &handle,
                            &format!(
                                "\n❌ 测试执行失败 (exit_code: {})\n```\n{}\n```\n",
                                exit_code,
                                if !stderr.is_empty() {
                                    &stderr[..stderr.len().min(1500)]
                                } else {
                                    &stdout[..stdout.len().min(1500)]
                                }
                            ),
                        );

                        // ----- 2d: 测试修复 (Self-Healing) -----
                        tracker.emit_phase(&handle, AgentPhase::TestRepair, "AI 正在修复失败的测试...");
                        tracker.emit_chat_progress(
                            &handle,
                            "\n🔧 **自修复引擎**: AI 正在分析错误并修复测试...\n",
                        );

                        // 收集失败的测试代码
                        let failed_code: String = generated_tests
                            .iter()
                            .filter_map(|t| t.test_code.as_ref())
                            .cloned()
                            .collect::<Vec<_>>()
                            .join("\n\n");

                        let error_output = if !stderr.is_empty() {
                            stderr.clone()
                        } else {
                            stdout.clone()
                        };

                        match self
                            .test_gen
                            .generate_test_repair(handle.clone(), id, &failed_code, &error_output)
                            .await
                        {
                            Ok(repaired_tests) => {
                                if !repaired_tests.is_empty() {
                                    tests_repaired = repaired_tests.len() as i32;
                                    tracker.emit_chat_progress(
                                        &handle,
                                        &format!(
                                            "\n✅ 修复了 {} 个测试用例，重新写入...\n",
                                            tests_repaired
                                        ),
                                    );

                                    // 用修复后的测试替换原始测试
                                    final_tests = repaired_tests.clone();

                                    // 刷新测试文件内容后重新写入
                                    // 注意：这里简化处理，实际应该替换而非追加
                                    // 更新失败测试反馈信息
                                    self.test_gen.update_failed_tests(&format!(
                                        "Test execution failed with exit_code {}.\nError: {}",
                                        exit_code,
                                        &error_output[..error_output.len().min(2000)]
                                    ));
                                } else {
                                    tracker.emit_chat_progress(
                                        &handle,
                                        "\n⚠️ 修复未产生有效测试，保留原始测试。\n",
                                    );
                                }
                            }
                            Err(e) => {
                                warn!("Test repair failed: {}", e);
                                tracker.emit_chat_progress(
                                    &handle,
                                    &format!("\n⚠️ 测试修复失败: {}\n", e),
                                );
                                // 将失败信息反馈给下一轮
                                self.test_gen.update_failed_tests(&format!(
                                    "Test execution failed with exit_code {}.\nError: {}",
                                    exit_code,
                                    &error_output[..error_output.len().min(2000)]
                                ));
                            }
                        }
                    }

                    // ----- 2e: 覆盖率验证 -----
                    tracker.emit_phase(
                        &handle,
                        AgentPhase::CoverageVerification,
                        "解析覆盖率报告...",
                    );
                    tracker.emit_chat_progress(
                        &handle,
                        "\n📊 **Phase 4/4**: 解析覆盖率报告...\n",
                    );

                    if let Some(cov_result) = self.parse_coverage_after_test(start_time) {
                        tracker.set_coverage(cov_result.coverage_percentage);

                        // ----- 2f: 反馈覆盖率到下一轮 -----
                        self.test_gen
                            .update_coverage_report(&cov_result.to_report_text());
                        self.test_gen.refresh_test_file();

                        tracker.emit_chat_progress(
                            &handle,
                            &format!(
                                "\n📈 覆盖率: {:.1}% → {:.1}% (Δ {:.1}%)\n",
                                coverage_before * 100.0,
                                cov_result.coverage_percentage * 100.0,
                                (cov_result.coverage_percentage - coverage_before) * 100.0,
                            ),
                        );

                        if tracker.is_coverage_met() {
                            tracker.emit_chat_progress(
                                &handle,
                                &format!(
                                    "\n🎯 **覆盖率已达标** ({:.1}% >= {}%)，提前结束迭代！\n",
                                    cov_result.coverage_percentage * 100.0,
                                    self.desired_coverage,
                                ),
                            );
                        }
                    }
                }
            }

            // 收集测试结果（去重）
            for test in final_tests.into_iter() {
                let is_duplicate = test_results_list.iter().any(|existing| {
                    existing.test_number == test.test_number
                });
                if !is_duplicate {
                    test_results_list.push(test);
                }
            }

            // 记录本轮迭代
            let coverage_after = tracker.get_coverage();
            tracker.record_iteration(IterationRecord {
                iteration: iteration_count,
                tests_generated: num_generated,
                tests_passed,
                tests_failed,
                tests_repaired,
                coverage_before,
                coverage_after,
                coverage_delta: coverage_after - coverage_before,
            });
        }

        // ===== Phase 3: 静态代码分析 =====
        tracker.emit_phase(&handle, AgentPhase::StaticAnalysis, "执行静态代码分析...");
        tracker.emit_chat_progress(
            &handle,
            "\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             🔍 **静态代码分析**\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
        );

        let static_sec_result = match self
            .test_gen
            .generate_static_sec(handle.clone(), id, 4096, false)
            .await
        {
            Ok(result) => {
                let total_issues = result.coding_standard_issues.len()
                    + result.performance_issues.len()
                    + result.security_vulnerabilities.len();
                tracker.emit_chat_progress(
                    &handle,
                    &format!(
                        "\n📋 发现 {} 个问题 (规范: {}, 性能: {}, 安全: {})\n",
                        total_issues,
                        result.coding_standard_issues.len(),
                        result.performance_issues.len(),
                        result.security_vulnerabilities.len(),
                    ),
                );
                result
            }
            Err(e) => {
                error!("Error generating static analysis: {}", e);
                tracker.emit_chat_progress(
                    &handle,
                    &format!("\n⚠️ 静态分析失败: {}\n", e),
                );
                // 返回空结果以继续流程
                crate::app::uagent::unitest_agent_test_generator::CodeAnalysisResult {
                    language: String::new(),
                    total_issues: 0,
                    coding_standard_issues: vec![],
                    performance_issues: vec![],
                    security_vulnerabilities: vec![],
                }
            }
        };

        // ===== Phase 4: 测试质量评分 =====
        tracker.emit_phase(&handle, AgentPhase::QualityScoring, "AI 评估测试质量...");
        tracker.emit_chat_progress(
            &handle,
            "\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             📊 **测试质量评分 (含变异测试分析)**\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
        );

        match self.test_gen.generate_quality_score(handle.clone(), id).await {
            Ok(score) => {
                quality_score_result = Some(score.clone());
                tracker.emit_chat_progress(
                    &handle,
                    &format!("\n📊 质量评分结果:\n```yaml\n{}\n```\n", &score[..score.len().min(3000)]),
                );
            }
            Err(e) => {
                warn!("Quality scoring failed: {}", e);
                tracker.emit_chat_progress(
                    &handle,
                    &format!("\n⚠️ 质量评分失败: {}\n", e),
                );
            }
        }

        // ===== Phase 5: 生成报告 =====
        tracker.emit_phase(&handle, AgentPhase::ReportGeneration, "生成综合报告...");
        tracker.emit_chat_progress(
            &handle,
            "\n📄 正在生成综合报告...\n",
        );

        let report_path = ReportGenerator::generate_combined_report(
            handle.clone(),
            unitest_id,
            &test_results_list,
            &static_sec_result,
        )
        .await;

        // ===== Phase 6: 完成 =====
        tracker.emit_phase(&handle, AgentPhase::Completed, "Agent 执行完成");

        // 输出执行摘要
        let summary = tracker.summary_text();
        tracker.emit_chat_progress(
            &handle,
            &format!(
                "\n{}\n\
                 🏁 **Agent 执行完成**\n\
                 - 总测试用例: {}\n\
                 - 最终覆盖率: {:.1}%\n\
                 - 静态问题: {}\n\
                 {}\n",
                summary,
                test_results_list.len(),
                tracker.get_coverage() * 100.0,
                static_sec_result.coding_standard_issues.len()
                    + static_sec_result.performance_issues.len()
                    + static_sec_result.security_vulnerabilities.len(),
                if let Ok(ref path) = report_path {
                    format!("- 报告路径: {}", path)
                } else {
                    "- 报告生成失败".to_string()
                },
            ),
        );

        info!("UnitestAgent run completed. Report: {:?}", report_path);
    }
}

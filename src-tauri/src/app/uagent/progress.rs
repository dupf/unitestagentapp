use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

// =========================================================================
// 结构化进度追踪系统
// 用于向前端实时推送 Agent 各阶段状态、覆盖率变化、迭代信息
// =========================================================================

/// Agent 执行阶段枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentPhase {
    /// 初始化：读取配置、检测语言
    Initializing,
    /// 覆盖率分析：解析现有覆盖率报告
    CoverageAnalysis,
    /// 测试生成：AI 生成测试用例
    TestGeneration,
    /// 测试写入：将生成的测试写入文件
    TestWriting,
    /// 测试执行：运行测试命令
    TestExecution,
    /// 测试修复：AI 修复失败的测试
    TestRepair,
    /// 覆盖率验证：重新解析覆盖率报告
    CoverageVerification,
    /// 静态代码分析
    StaticAnalysis,
    /// 质量评分：AI 评估测试质量
    QualityScoring,
    /// 变异测试分析
    MutationAnalysis,
    /// 报告生成
    ReportGeneration,
    /// 已完成
    Completed,
    /// 失败
    Failed,
}

impl std::fmt::Display for AgentPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentPhase::Initializing => write!(f, "初始化"),
            AgentPhase::CoverageAnalysis => write!(f, "覆盖率分析"),
            AgentPhase::TestGeneration => write!(f, "AI 测试生成"),
            AgentPhase::TestWriting => write!(f, "写入测试文件"),
            AgentPhase::TestExecution => write!(f, "执行测试"),
            AgentPhase::TestRepair => write!(f, "AI 测试修复"),
            AgentPhase::CoverageVerification => write!(f, "覆盖率验证"),
            AgentPhase::StaticAnalysis => write!(f, "静态代码分析"),
            AgentPhase::QualityScoring => write!(f, "测试质量评分"),
            AgentPhase::MutationAnalysis => write!(f, "变异测试分析"),
            AgentPhase::ReportGeneration => write!(f, "生成报告"),
            AgentPhase::Completed => write!(f, "已完成"),
            AgentPhase::Failed => write!(f, "失败"),
        }
    }
}

/// 单次迭代的执行记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationRecord {
    pub iteration: i32,
    pub tests_generated: i32,
    pub tests_passed: i32,
    pub tests_failed: i32,
    pub tests_repaired: i32,
    pub coverage_before: f64,
    pub coverage_after: f64,
    pub coverage_delta: f64,
}

/// Agent 进度负载（发送给前端的事件数据）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProgressPayload {
    pub id: u64,
    pub phase: AgentPhase,
    pub iteration: i32,
    pub max_iterations: i32,
    pub current_coverage: f64,
    pub desired_coverage: f64,
    pub message: String,
    pub detail: String,
}

const AGENT_PROGRESS_EVENT: &str = "AGENT_PROGRESS";

/// 进度追踪器
pub struct ProgressTracker {
    id: u64,
    max_iterations: i32,
    desired_coverage: f64,
    current_iteration: i32,
    current_coverage: f64,
    pub iteration_records: Vec<IterationRecord>,
}

impl ProgressTracker {
    pub fn new(id: u64, max_iterations: i32, desired_coverage: f64) -> Self {
        ProgressTracker {
            id,
            max_iterations,
            desired_coverage,
            current_iteration: 0,
            current_coverage: 0.0,
            iteration_records: Vec::new(),
        }
    }

    /// 发送阶段进度事件到前端
    pub fn emit_phase(&self, handle: &AppHandle, phase: AgentPhase, message: &str) {
        let payload = AgentProgressPayload {
            id: self.id,
            phase: phase.clone(),
            iteration: self.current_iteration,
            max_iterations: self.max_iterations,
            current_coverage: self.current_coverage,
            desired_coverage: self.desired_coverage,
            message: message.to_string(),
            detail: format!(
                "[迭代 {}/{}] {} | 覆盖率: {:.1}% / {:.1}%",
                self.current_iteration,
                self.max_iterations,
                phase,
                self.current_coverage * 100.0,
                self.desired_coverage as f64,
            ),
        };
        handle.emit_all(AGENT_PROGRESS_EVENT, &payload).ok();
    }

    /// 发送聊天式进度消息（兼容现有 CHAT_FETCHEING_PROGRESS 事件）
    pub fn emit_chat_progress(&self, handle: &AppHandle, message: &str) {
        let payload = super::unitest_agent_test_generator::ProgressPayload {
            id: self.id,
            detail: message.to_string(),
            role: "assistant".to_string(),
            finish_reason: "".to_string(),
        };
        payload.emit_progress(handle);
    }

    /// 更新当前迭代号
    pub fn set_iteration(&mut self, iteration: i32) {
        self.current_iteration = iteration;
    }

    /// 更新当前覆盖率
    pub fn set_coverage(&mut self, coverage: f64) {
        self.current_coverage = coverage;
    }

    /// 记录一次迭代结果
    pub fn record_iteration(&mut self, record: IterationRecord) {
        self.iteration_records.push(record);
    }

    /// 检查是否已达到目标覆盖率
    pub fn is_coverage_met(&self) -> bool {
        self.current_coverage * 100.0 >= self.desired_coverage as f64
    }

    /// 获取当前覆盖率
    pub fn get_coverage(&self) -> f64 {
        self.current_coverage
    }

    /// 生成最终的进度摘要文本
    pub fn summary_text(&self) -> String {
        let mut summary = String::new();
        summary.push_str(&format!(
            "\n===== Agent 执行摘要 =====\n"
        ));
        summary.push_str(&format!(
            "总迭代次数: {}\n",
            self.iteration_records.len()
        ));
        summary.push_str(&format!(
            "最终覆盖率: {:.1}%\n",
            self.current_coverage * 100.0
        ));
        summary.push_str(&format!(
            "目标覆盖率: {:.1}%\n",
            self.desired_coverage as f64
        ));
        summary.push_str(&format!(
            "达标状态: {}\n",
            if self.is_coverage_met() { "已达标" } else { "未达标" }
        ));

        let total_generated: i32 = self.iteration_records.iter().map(|r| r.tests_generated).sum();
        let total_passed: i32 = self.iteration_records.iter().map(|r| r.tests_passed).sum();
        let total_failed: i32 = self.iteration_records.iter().map(|r| r.tests_failed).sum();
        let total_repaired: i32 = self.iteration_records.iter().map(|r| r.tests_repaired).sum();

        summary.push_str(&format!("测试用例生成: {}\n", total_generated));
        summary.push_str(&format!("测试通过: {}\n", total_passed));
        summary.push_str(&format!("测试失败: {}\n", total_failed));
        summary.push_str(&format!("测试修复: {}\n", total_repaired));

        for record in &self.iteration_records {
            summary.push_str(&format!(
                "\n--- 迭代 {} ---\n  生成: {} | 通过: {} | 失败: {} | 修复: {}\n  覆盖率: {:.1}% → {:.1}% (Δ {:.1}%)\n",
                record.iteration,
                record.tests_generated,
                record.tests_passed,
                record.tests_failed,
                record.tests_repaired,
                record.coverage_before * 100.0,
                record.coverage_after * 100.0,
                record.coverage_delta * 100.0,
            ));
        }

        summary
    }
}

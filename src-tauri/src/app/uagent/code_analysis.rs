#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeIssueCn {
    #[serde(rename = "问题编号")]
    pub issue_number: String,
    
    #[serde(rename = "类别")]
    pub category: String,
    
    #[serde(rename = "严重性")]
    pub severity: String,
    
    #[serde(rename = "描述")]
    pub description: String,
    
    #[serde(rename = "位置")]
    pub location: String,
    
    #[serde(rename = "影响")]
    pub impact: String,
    
    #[serde(rename = "建议")]
    pub suggestion: String,
    
    #[serde(rename = "最佳实践")]
    pub best_practice: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeAnalysisCn {
    #[serde(rename = "语言")]
    pub language: String,
    
    #[serde(rename = "问题总数")]
    pub total_issues: i32,
    
    #[serde(rename = "安全漏洞")]
    pub security_vulnerabilities: Vec<CodeIssueCn>,
    
    #[serde(rename = "代码质量问题")]
    pub code_quality_issues: Vec<CodeIssueCn>,
    
    #[serde(rename = "性能问题")]
    pub performance_issues: Vec<CodeIssueCn>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeIssueEn {
    #[serde(rename = "issue_number")]
    pub issue_number: String,
    
    #[serde(rename = "category")]
    pub category: String,
    
    #[serde(rename = "severity")]
    pub severity: String,
    
    #[serde(rename = "description")]
    pub description: String,
    
    #[serde(rename = "location")]
    pub location: String,
    
    #[serde(rename = "impact")]
    pub impact: String,
    
    #[serde(rename = "suggestion")]
    pub suggestion: String,
    
    #[serde(rename = "best_practice")]
    pub best_practice: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeAnalysisEn {
    #[serde(rename = "language")]
    pub language: String,
    
    #[serde(rename = "total_issues")]
    pub total_issues: i32,
    
    #[serde(rename = "security_vulnerabilities")]
    pub security_vulnerabilities: Vec<CodeIssueEn>,
    
    #[serde(rename = "code_quality_issues")]
    pub code_quality_issues: Vec<CodeIssueEn>,
    
    #[serde(rename = "performance_issues")]
    pub performance_issues: Vec<CodeIssueEn>,
}

impl CodeIssueCn {
    pub fn to_english(&self) -> CodeIssueEn {
        CodeIssueEn {
            issue_number: self.issue_number.clone(),
            category: self.category.clone(),
            severity: self.severity.clone(),
            description: self.description.clone(),
            location: self.location.clone(),
            impact: self.impact.clone(),
            suggestion: self.suggestion.clone(),
            best_practice: self.best_practice.clone(),
        }
    }
}

impl CodeAnalysisCn {
    pub fn to_english(&self) -> CodeAnalysisEn {
        let security_vulnerabilities_en: Vec<CodeIssueEn> = self.security_vulnerabilities
            .iter()
            .map(|issue| issue.to_english())
            .collect();
            
        let code_quality_issues_en: Vec<CodeIssueEn> = self.code_quality_issues
            .iter()
            .map(|issue| issue.to_english())
            .collect();
            
        let performance_issues_en: Vec<CodeIssueEn> = self.performance_issues
            .iter()
            .map(|issue| issue.to_english())
            .collect();
            
        CodeAnalysisEn {
            language: self.language.clone(),
            total_issues: self.total_issues,
            security_vulnerabilities: security_vulnerabilities_en,
            code_quality_issues: code_quality_issues_en,
            performance_issues: performance_issues_en,
        }
    }
}
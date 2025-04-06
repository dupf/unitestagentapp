use crate::app::uagent::unitest_agent_test_generator::{
    CodeAnalysisResult, CodeAnalysisResultEn, CodeIssueEn, TestDetails, TestDetailsEn
};
use serde::Serialize;
use std::fs;
use tauri::api::path::{app_data_dir, resource_dir};
use tauri::{AppHandle, Manager};
use tera::{Context, Tera};

#[derive(Serialize)]
struct SimpleTestResult {
    test_number: String,
}
pub struct ReportGenerator {
    template: &'static str,
}

#[derive(Serialize, Debug)]
pub struct TestResultsoutput {
    pub test_number: String,
    pub test_technique: String,
    pub test_description: String,
    pub test_code: String,
    pub global_variables: String,
    pub initialization_code: String,
    pub stub_functions: String,
    pub input: String,
    pub expected_output: String,
    pub actual_output: String,
    pub conclusion: String,
}

impl ReportGenerator {
    const HTML_TEMPLATE: &'static str = r#"
     <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Test Results</title>
        <link href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.23.0/themes/prism-okaidia.min.css" rel="stylesheet" />
        <style>
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                margin: 20px;
            }
            table {
                border-collapse: collapse;
                width: 100%;
                box-shadow: 0 2px 3px rgba(0,0,0,0.1);
            }
            th, td {
                border: 1px solid #ddd;
                text-align: left;
                padding: 8px;
            }
            th {
                background-color: #f2f2f2;
            }
            tr:nth-child(even) {
                background-color: #f9f9f9;
            }
            .status-pass {
                color: green;
            }
            .status-fail {
                color: red;
            }
            pre {
                background-color: #000000 !important;
                color: #ffffff !important;
                padding: 5px;
                border-radius: 5px;
            }
        </style>
    </head>
    <body>
        <table>
            <tr>
                <th>测试用例编号</th>
                <th>测试技术</th>
                <th>测试用例描述</th>
                <th>测试代码</th>
                <th>全局变量</th>
                <th>初始化代码</th>
                <th>桩函数</th>
                <th>输入</th>
                <th>预期输出</th>
                <th>实际输出</th>
                <th>结论</th>
            </tr>
            {% for result in results %}
            <tr>
                <td>{{ result.test_number }}</td>
                <td>{{ result.test_technique }}</td>
                <td>{{ result.test_description }}</td>
                <td>{{ result.test_code }}</td>
                <td>{{ result.global_variables }}</td>
                <td>{{ result.initialization_code }}</td>
                <td>{{ result.stub_functions }}</td>
                <td>{{ result.input }}</td>
                <td>{{ result.expected_output }}</td>
                <td>{{ result.actual_output }}</td>
                <td>{{ result.conclusion }}</td>
            </tr>
            {% endfor %}
        </table>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.23.0/prism.min.js"></script>
    </body>
    </html>
    "#;

    // 添加静态代码分析报告的HTML模板
    const STATIC_ANALYSIS_TEMPLATE: &'static str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>静态代码分析报告</title>
        <style>
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                margin: 20px;
                color: #333;
                line-height: 1.6;
            }
            h1, h2 {
                color: #444;
                border-bottom: 1px solid #ddd;
                padding-bottom: 10px;
                margin-top: 20px;
            }
            .summary-card {
                display: flex;
                justify-content: space-between;
                margin-top: 20px;
            }
            .summary-item {
                border: 1px solid #ddd;
                border-radius: 5px;
                padding: 15px;
                flex: 1;
                margin: 0 10px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }
            .summary-item h3 {
                margin-top: 0;
                color: #0066cc;
            }
            .issues-container {
                margin-top: 20px;
            }
            .issue-card {
                border: 1px solid #ddd;
                border-radius: 5px;
                padding: 15px;
                margin-bottom: 15px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }
            .issue-header {
                display: flex;
                justify-content: space-between;
                border-bottom: 1px solid #eee;
                padding-bottom: 10px;
                margin-bottom: 10px;
            }
            .issue-title {
                font-weight: bold;
                font-size: 1.1em;
            }
            .severity-high {
                color: #d9534f;
            }
            .severity-medium {
                color: #f0ad4e;
            }
            .severity-low {
                color: #5bc0de;
            }
            .recommendation {
                background-color: #f9f9f9;
                padding: 10px;
                border-left: 3px solid #0066cc;
                margin-top: 10px;
            }
            .summary-total {
                font-size: 1.5em;
                text-align: center;
                margin: 20px 0;
                font-weight: bold;
            }
            ul.stats {
                list-style: none;
                padding: 0;
            }
            ul.stats li {
                padding: 8px 10px;
                margin-bottom: 10px;
                border-radius: 4px;
                background-color: #f5f5f5;
            }
            ul.stats li span {
                float: right;
                font-weight: bold;
            }
        </style>
    </head>
    <body>
        <h1>静态代码分析报告</h1>
        <p>分析语言: {{ analysis.language }}</p>
        <div class="summary-total">总计发现 {{ analysis.total_issues }} 个问题</div>
        
        <div class="summary-card">
            <div class="summary-item">
                <h3>问题统计</h3>
                <ul class="stats">
                    <li>代码规范问题：<span>{{ analysis.coding_standard_issues | length }}处</span></li>
                    // <li>潜在缺陷：<span>{{ analysis.coding_standard_issues | length }}处</span></li>
                    <li>性能问题：<span>{{ analysis.performance_issues | length }}处</span></li>
                    <li>安全漏洞：<span>{{ analysis.security_vulnerabilities | length }}处</span></li>
                </ul>
            </div>
        </div>

        {% if analysis.coding_standard_issues | length > 0 %}
        <h2>代码规范问题</h2>
        <div class="issues-container">
            {% for issue in analysis.coding_standard_issues %}
            <div class="issue-card">
                <div class="issue-header">
                    <div class="issue-title">{{ issue.issue_id }} - {{ issue.description }}</div>
                    <div class="severity {% if issue.severity == '高' or issue.severity == 'high' %}severity-high{% elif issue.severity == '中' or issue.severity == 'medium' %}severity-medium{% else %}severity-low{% endif %}">
                        严重性: {{ issue.severity }}
                    </div>
                </div>
                <p><strong>位置:</strong> {{ issue.location }}</p>
                <p><strong>影响:</strong> {{ issue.impact }}</p>
                <div class="recommendation">
                    <strong>修复建议:</strong><br>
                    {{ issue.recommendation | replace(from="\n", to="<br>") | safe }}
                </div>
                <p><strong>最佳实践:</strong> {{ issue.best_practice }}</p>
            </div>
            {% endfor %}
        </div>
        {% endif %}

        {% if analysis.performance_issues | length > 0 %}
        <h2>性能问题</h2>
        <div class="issues-container">
            {% for issue in analysis.performance_issues %}
            <div class="issue-card">
                <div class="issue-header">
                    <div class="issue-title">{{ issue.issue_id }} - {{ issue.description }}</div>
                    <div class="severity {% if issue.severity == '高' or issue.severity == 'high' %}severity-high{% elif issue.severity == '中' or issue.severity == 'medium' %}severity-medium{% else %}severity-low{% endif %}">
                        严重性: {{ issue.severity }}
                    </div>
                </div>
                <p><strong>位置:</strong> {{ issue.location }}</p>
                <p><strong>影响:</strong> {{ issue.impact }}</p>
                <div class="recommendation">
                    <strong>修复建议:</strong><br>
                    {{ issue.recommendation | replace(from="\n", to="<br>") | safe }}
                </div>
                <p><strong>最佳实践:</strong> {{ issue.best_practice }}</p>
            </div>
            {% endfor %}
        </div>
        {% endif %}

        {% if analysis.security_vulnerabilities | length > 0 %}
        <h2>安全漏洞</h2>
        <div class="issues-container">
            {% for issue in analysis.security_vulnerabilities %}
            <div class="issue-card">
                <div class="issue-header">
                    <div class="issue-title">{{ issue.issue_id }} - {{ issue.description }}</div>
                    <div class="severity {% if issue.severity == '高' or issue.severity == 'high' %}severity-high{% elif issue.severity == '中' or issue.severity == 'medium' %}severity-medium{% else %}severity-low{% endif %}">
                        严重性: {{ issue.severity }}
                    </div>
                </div>
                <p><strong>位置:</strong> {{ issue.location }}</p>
                <p><strong>影响:</strong> {{ issue.impact }}</p>
                <div class="recommendation">
                    <strong>修复建议:</strong><br>
                    {{ issue.recommendation | replace(from="\n", to="<br>") | safe }}
                </div>
                <p><strong>最佳实践:</strong> {{ issue.best_practice }}</p>
            </div>
            {% endfor %}
        </div>
        {% endif %}
    </body>
    </html>
    "#;

    pub async fn generate_report(
        handle: AppHandle,
        results: &Vec<TestDetails>,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let results_en: Vec<TestDetailsEn> =
            results.iter().map(|detail| detail.to_english()).collect();

        let mut tera = Tera::default();
        if let Err(e) = tera.add_raw_template("report", Self::HTML_TEMPLATE) {
            println!("Error adding raw template: {:?}", e);
            return Err(Box::new(e));
        }
        let mut context = Context::new();
        context.insert("results", &results_en);

        let html_content = match tera.render("report", &context) {
            Ok(content) => content,
            Err(e) => {
                println!("模板渲染错误: {:?}", e);
                return Err(Box::new(e));
            }
        };
        // 保存到应用数据目录（可写目录）
        let app_data = app_data_dir(&handle.config()).expect("无法获取应用数据目录");
        let reports_dir = app_data.join("reports");
        // println!("app_data===={:?}", app_data);
        // println!("reports_dir====={:?}", reports_dir);
        // 确保报告目录存在
        fs::create_dir_all(&reports_dir)?;
        // 创建报告文件名（使用时间戳确保唯一性）
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let file_name = format!("report_{}.html", timestamp);
        let report_path = reports_dir.join(&file_name);

        // 写入文件
        fs::write(&report_path, html_content)?;

        // 返回生成的报告路径（相对于app_data_dir的路径）
        let relative_path = format!("reports/{}", file_name);
        print!("relative_path===={}", relative_path);

        Ok(())
    }

    // 新增静态代码分析报告生成函数
    pub async fn generate_static_analysis_report(
        handle: AppHandle,
        analysis: &CodeAnalysisResult,
    ) -> Result<String, Box<dyn std::error::Error>> {

        let analysisEN = CodeAnalysisResultEn {
            language: analysis.language.clone(),
            total_issues: analysis.total_issues,
            coding_standard_issues: analysis.coding_standard_issues.iter().map(|issue| CodeIssueEn {
                issue_id: issue.issue_id.clone(),
                category: issue.category.clone(),
                severity: issue.severity.clone(),
                description: issue.description.clone(),
                location: issue.location.clone(),
                impact: issue.impact.clone(),
                recommendation: issue.recommendation.clone(),
                best_practice: issue.best_practice.clone(),
            }).collect(),
            performance_issues: analysis.performance_issues.iter().map(|issue| CodeIssueEn {
                issue_id: issue.issue_id.clone(),
                category: issue.category.clone(),
                severity: issue.severity.clone(),
                description: issue.description.clone(),
                location: issue.location.clone(),
                impact: issue.impact.clone(),
                recommendation: issue.recommendation.clone(),
                best_practice: issue.best_practice.clone(),
            }).collect(),
            security_vulnerabilities: analysis.security_vulnerabilities.iter().map(|issue| CodeIssueEn {
                issue_id: issue.issue_id.clone(),
                category: issue.category.clone(),
                severity: issue.severity.clone(),
                description: issue.description.clone(),
                location: issue.location.clone(),
                impact: issue.impact.clone(),
                recommendation: issue.recommendation.clone(),
                best_practice: issue.best_practice.clone(),
            }).collect(),
        };


        let mut tera = Tera::default();
        if let Err(e) = tera.add_raw_template("report", Self::STATIC_ANALYSIS_TEMPLATE) {
            println!("Error adding raw template: {:?}", e);
            return Err(Box::new(e));
        }

        let mut context = Context::new();

        // 将 CodeAnalysisResult 转换为英文版本的数据结构
        // 为了简化，我们直接使用原始结构
        println!("Analysis Information: {:?}", analysis);
        context.insert("analysis", &analysisEN);

        // 渲染HTML
        let html_content = match tera.render("report", &context) {
            Ok(content) => content,
            Err(e) => {
                println!("模板渲染错误: {:?}", e);
                return Err(Box::new(e));
            }
        };

        // 保存到应用数据目录（可写目录）
        let app_data = app_data_dir(&handle.config()).expect("无法获取应用数据目录");
        let reports_dir = app_data.join("reports");

        println!("app_data===={:?}", app_data);
        println!("reports_dir====={:?}", reports_dir);

        // 确保报告目录存在
        fs::create_dir_all(&reports_dir)?;

        // 创建报告文件名（使用时间戳确保唯一性）
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let file_name = format!("static_analysis_report_{}.html", timestamp);
        let report_path = reports_dir.join(&file_name);

        // 写入文件
        fs::write(&report_path, html_content)?;

        // 返回生成的报告文件路径
        Ok(report_path.to_string_lossy().to_string())
    }
}

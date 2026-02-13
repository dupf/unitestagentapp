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
                    {{ issue.recommendation | replace("\n", "<br>") | safe }}
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
                    {{ issue.recommendation | replace("\n", "<br>") | safe }}
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
                    {{ issue.recommendation | replace("\n", "<br>") | safe }}
                </div>
                <p><strong>最佳实践:</strong> {{ issue.best_practice }}</p>
            </div>
            {% endfor %}
        </div>
        {% endif %}
    </body>
    </html>
    "#;

    // =========================================================================
    // 增强版综合报告模板 v2
    // 新增: 覆盖率仪表盘、迭代追踪、质量评分面板
    // =========================================================================
    const COMBINED_TEMPLATE: &'static str = r###"
    <!DOCTYPE html>
    <html lang="zh-CN">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>UnitestAgent 综合测试报告</title>
        <link href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css" rel="stylesheet" />
        <style>
            :root {
                --bg: #f8fafc;
                --card-bg: #ffffff;
                --border: #e2e8f0;
                --text: #1e293b;
                --text-muted: #64748b;
                --primary: #3b82f6;
                --success: #22c55e;
                --warning: #f59e0b;
                --danger: #ef4444;
                --info: #06b6d4;
            }
            * { margin: 0; padding: 0; box-sizing: border-box; }
            body {
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                background: var(--bg);
                color: var(--text);
                line-height: 1.6;
            }
            .container { max-width: 1400px; margin: 0 auto; padding: 24px; }
            .header {
                background: linear-gradient(135deg, #1e40af 0%, #3b82f6 50%, #06b6d4 100%);
                color: white;
                padding: 32px;
                border-radius: 16px;
                margin-bottom: 24px;
            }
            .header h1 { font-size: 28px; font-weight: 700; margin-bottom: 8px; }
            .header .subtitle { opacity: 0.85; font-size: 14px; }

            /* 导航标签页 */
            .tabs {
                display: flex;
                gap: 4px;
                background: var(--card-bg);
                border-radius: 12px;
                padding: 4px;
                margin-bottom: 24px;
                border: 1px solid var(--border);
                overflow-x: auto;
            }
            .tabs a {
                padding: 10px 20px;
                border-radius: 8px;
                text-decoration: none;
                color: var(--text-muted);
                font-weight: 500;
                font-size: 14px;
                transition: all 0.2s;
                white-space: nowrap;
                cursor: pointer;
            }
            .tabs a:hover { background: #f1f5f9; color: var(--text); }
            .tabs a.active { background: var(--primary); color: white; }

            .tab { display: none; }
            .tab.active { display: block; }

            /* 仪表盘卡片 */
            .dashboard {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                gap: 16px;
                margin-bottom: 24px;
            }
            .metric-card {
                background: var(--card-bg);
                border-radius: 12px;
                padding: 20px;
                border: 1px solid var(--border);
                text-align: center;
            }
            .metric-card .metric-value {
                font-size: 36px;
                font-weight: 700;
                line-height: 1;
                margin-bottom: 8px;
            }
            .metric-card .metric-label {
                font-size: 13px;
                color: var(--text-muted);
                font-weight: 500;
            }
            .metric-card.primary .metric-value { color: var(--primary); }
            .metric-card.success .metric-value { color: var(--success); }
            .metric-card.warning .metric-value { color: var(--warning); }
            .metric-card.danger .metric-value { color: var(--danger); }
            .metric-card.info .metric-value { color: var(--info); }

            /* 通用卡片 */
            .card {
                background: var(--card-bg);
                border: 1px solid var(--border);
                border-radius: 12px;
                padding: 24px;
                margin-bottom: 16px;
            }
            .card h2 { font-size: 18px; margin-bottom: 16px; }
            .card h3 { font-size: 16px; margin-bottom: 12px; color: var(--text); }

            /* 问题卡片 */
            .issue-card {
                background: var(--card-bg);
                border: 1px solid var(--border);
                border-left: 4px solid var(--border);
                border-radius: 8px;
                padding: 16px;
                margin-bottom: 12px;
            }
            .issue-card.severity-high { border-left-color: var(--danger); }
            .issue-card.severity-medium { border-left-color: var(--warning); }
            .issue-card.severity-low { border-left-color: var(--info); }
            .issue-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 8px;
            }
            .issue-title { font-weight: 600; font-size: 15px; }
            .badge {
                display: inline-block;
                padding: 2px 10px;
                border-radius: 20px;
                font-size: 12px;
                font-weight: 600;
            }
            .badge-high { background: #fef2f2; color: var(--danger); }
            .badge-medium { background: #fffbeb; color: var(--warning); }
            .badge-low { background: #ecfeff; color: var(--info); }
            .recommendation {
                background: #f0f9ff;
                border-left: 3px solid var(--primary);
                padding: 12px;
                border-radius: 4px;
                margin-top: 8px;
                font-size: 14px;
            }

            /* 表格 */
            table {
                width: 100%;
                border-collapse: collapse;
                font-size: 14px;
            }
            table th {
                background: #f8fafc;
                font-weight: 600;
                padding: 12px;
                text-align: left;
                border-bottom: 2px solid var(--border);
                white-space: nowrap;
            }
            table td {
                padding: 10px 12px;
                border-bottom: 1px solid var(--border);
                vertical-align: top;
            }
            table tr:hover { background: #f8fafc; }

            /* 代码块 */
            pre[class*="language-"] {
                border-radius: 8px;
                font-size: 13px;
                margin: 8px 0;
            }
            td pre {
                max-height: 200px;
                overflow-y: auto;
                margin: 0;
            }
            code { font-family: 'SF Mono', 'Fira Code', 'Consolas', monospace; }

            /* 覆盖率进度条 */
            .coverage-bar {
                width: 100%;
                height: 24px;
                background: #e2e8f0;
                border-radius: 12px;
                overflow: hidden;
                margin: 8px 0;
            }
            .coverage-fill {
                height: 100%;
                border-radius: 12px;
                transition: width 1s ease;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 12px;
                font-weight: 600;
                color: white;
            }
            .fill-success { background: linear-gradient(90deg, #22c55e, #16a34a); }
            .fill-warning { background: linear-gradient(90deg, #f59e0b, #d97706); }
            .fill-danger { background: linear-gradient(90deg, #ef4444, #dc2626); }

            /* 时间线 */
            .timeline { position: relative; padding-left: 28px; }
            .timeline::before {
                content: '';
                position: absolute;
                left: 10px;
                top: 0;
                bottom: 0;
                width: 2px;
                background: var(--border);
            }
            .timeline-item {
                position: relative;
                padding: 12px 0;
            }
            .timeline-item::before {
                content: '';
                position: absolute;
                left: -22px;
                top: 16px;
                width: 10px;
                height: 10px;
                border-radius: 50%;
                background: var(--primary);
                border: 2px solid white;
            }
            .timeline-item.success::before { background: var(--success); }
            .timeline-item.warning::before { background: var(--warning); }

            @media (max-width: 768px) {
                .dashboard { grid-template-columns: repeat(2, 1fr); }
                .container { padding: 12px; }
            }
        </style>
    </head>
    <body>
        <div class="container">
            <!-- 头部 -->
            <div class="header">
                <h1>UnitestAgent 综合测试报告</h1>
                <div class="subtitle">AI 驱动的单元测试生成 & 静态代码分析 | Generated by UnitestAgent v0.1.0</div>
            </div>

            <!-- 导航 -->
            <div class="tabs">
                <a class="active" onclick="showTab('summary', this)">📊 总览</a>
                <a onclick="showTab('tests', this)">🧪 单元测试</a>
                <a onclick="showTab('analysis', this)">🔍 静态分析</a>
                <a onclick="showTab('coverage', this)">📈 覆盖率</a>
            </div>

            <!-- ====== 总览面板 ====== -->
            <div id="summary" class="tab active">
                <div class="dashboard">
                    <div class="metric-card primary">
                        <div class="metric-value">{{ test_results | length }}</div>
                        <div class="metric-label">生成测试用例</div>
                    </div>
                    <div class="metric-card success">
                        <div class="metric-value">{{ analysis.total_issues }}</div>
                        <div class="metric-label">静态分析问题</div>
                    </div>
                    <div class="metric-card warning">
                        <div class="metric-value">{{ analysis.coding_standard_issues | length }}</div>
                        <div class="metric-label">代码规范问题</div>
                    </div>
                    <div class="metric-card danger">
                        <div class="metric-value">{{ analysis.security_vulnerabilities | length }}</div>
                        <div class="metric-label">安全漏洞</div>
                    </div>
                    <div class="metric-card info">
                        <div class="metric-value">{{ analysis.performance_issues | length }}</div>
                        <div class="metric-label">性能问题</div>
                    </div>
                </div>

                <div class="card">
                    <h2>问题分布</h2>
                    <table>
                        <tr>
                            <th>类别</th>
                            <th>数量</th>
                            <th>占比</th>
                        </tr>
                        <tr>
                            <td>🔧 代码规范</td>
                            <td>{{ analysis.coding_standard_issues | length }}</td>
                            <td>
                                {% if analysis.total_issues > 0 %}
                                    {{ analysis.coding_standard_issues | length }}
                                {% else %}
                                    0
                                {% endif %}
                            </td>
                        </tr>
                        <tr>
                            <td>⚡ 性能问题</td>
                            <td>{{ analysis.performance_issues | length }}</td>
                            <td>
                                {% if analysis.total_issues > 0 %}
                                    {{ analysis.performance_issues | length }}
                                {% else %}
                                    0
                                {% endif %}
                            </td>
                        </tr>
                        <tr>
                            <td>🛡️ 安全漏洞</td>
                            <td>{{ analysis.security_vulnerabilities | length }}</td>
                            <td>
                                {% if analysis.total_issues > 0 %}
                                    {{ analysis.security_vulnerabilities | length }}
                                {% else %}
                                    0
                                {% endif %}
                            </td>
                        </tr>
                    </table>
                </div>
            </div>

            <!-- ====== 单元测试面板 ====== -->
            <div id="tests" class="tab">
                <div class="card">
                    <h2>🧪 AI 生成的单元测试用例 ({{ test_results | length }})</h2>
                    <div style="overflow-x: auto;">
                        <table>
                            <tr>
                                <th>编号</th>
                                <th>测试技术</th>
                                <th>描述</th>
                                <th>测试代码</th>
                                <th>输入</th>
                                <th>预期输出</th>
                                <th>结论</th>
                            </tr>
                            {% for result in test_results %}
                            <tr>
                                <td><strong>{{ result.test_number }}</strong></td>
                                <td>{{ result.test_technique }}</td>
                                <td>{{ result.test_description }}</td>
                                <td><pre><code>{{ result.test_code }}</code></pre></td>
                                <td>{{ result.input }}</td>
                                <td>{{ result.expected_output }}</td>
                                <td>
                                    {% if result.conclusion %}
                                        {% if result.conclusion is containing("PASS") %}
                                            <span class="badge" style="background:#dcfce7;color:#16a34a;">PASS</span>
                                        {% elif result.conclusion is containing("FAIL") %}
                                            <span class="badge badge-high">FAIL</span>
                                        {% else %}
                                            <span class="badge" style="background:#f1f5f9;color:#64748b;">{{ result.conclusion }}</span>
                                        {% endif %}
                                    {% endif %}
                                </td>
                            </tr>
                            {% endfor %}
                        </table>
                    </div>
                </div>
            </div>

            <!-- ====== 静态分析面板 ====== -->
            <div id="analysis" class="tab">
                {% if analysis.coding_standard_issues | length > 0 %}
                <div class="card">
                    <h2>🔧 代码规范问题 ({{ analysis.coding_standard_issues | length }})</h2>
                    {% for issue in analysis.coding_standard_issues %}
                    <div class="issue-card {% if issue.severity is containing('高') or issue.severity is containing('high') %}severity-high{% elif issue.severity is containing('中') or issue.severity is containing('medium') %}severity-medium{% else %}severity-low{% endif %}">
                        <div class="issue-header">
                            <span class="issue-title">{{ issue.issue_id }} — {{ issue.description }}</span>
                            <span class="badge {% if issue.severity is containing('高') or issue.severity is containing('high') %}badge-high{% elif issue.severity is containing('中') or issue.severity is containing('medium') %}badge-medium{% else %}badge-low{% endif %}">
                                {{ issue.severity }}
                            </span>
                        </div>
                        <p style="color:var(--text-muted);font-size:13px;">📍 {{ issue.location }}</p>
                        <p style="margin-top:6px;"><strong>影响:</strong> {{ issue.impact }}</p>
                        <div class="recommendation">
                            <strong>💡 修复建议:</strong><br>
                            {{ issue.recommendation | replace(from="\n", to="<br>") | safe }}
                        </div>
                        {% if issue.best_practice %}<p style="margin-top:8px;font-size:13px;color:var(--text-muted);">📚 最佳实践: {{ issue.best_practice }}</p>{% endif %}
                    </div>
                    {% endfor %}
                </div>
                {% endif %}

                {% if analysis.performance_issues | length > 0 %}
                <div class="card">
                    <h2>⚡ 性能问题 ({{ analysis.performance_issues | length }})</h2>
                    {% for issue in analysis.performance_issues %}
                    <div class="issue-card {% if issue.severity is containing('高') or issue.severity is containing('high') %}severity-high{% elif issue.severity is containing('中') or issue.severity is containing('medium') %}severity-medium{% else %}severity-low{% endif %}">
                        <div class="issue-header">
                            <span class="issue-title">{{ issue.issue_id }} — {{ issue.description }}</span>
                            <span class="badge {% if issue.severity is containing('高') or issue.severity is containing('high') %}badge-high{% elif issue.severity is containing('中') or issue.severity is containing('medium') %}badge-medium{% else %}badge-low{% endif %}">
                                {{ issue.severity }}
                            </span>
                        </div>
                        <p style="color:var(--text-muted);font-size:13px;">📍 {{ issue.location }}</p>
                        <p style="margin-top:6px;"><strong>影响:</strong> {{ issue.impact }}</p>
                        <div class="recommendation">
                            <strong>💡 修复建议:</strong><br>
                            {{ issue.recommendation | replace(from="\n", to="<br>") | safe }}
                        </div>
                        {% if issue.best_practice %}<p style="margin-top:8px;font-size:13px;color:var(--text-muted);">📚 最佳实践: {{ issue.best_practice }}</p>{% endif %}
                    </div>
                    {% endfor %}
                </div>
                {% endif %}

                {% if analysis.security_vulnerabilities | length > 0 %}
                <div class="card">
                    <h2>🛡️ 安全漏洞 ({{ analysis.security_vulnerabilities | length }})</h2>
                    {% for issue in analysis.security_vulnerabilities %}
                    <div class="issue-card {% if issue.severity is containing('高') or issue.severity is containing('high') %}severity-high{% elif issue.severity is containing('中') or issue.severity is containing('medium') %}severity-medium{% else %}severity-low{% endif %}">
                        <div class="issue-header">
                            <span class="issue-title">{{ issue.issue_id }} — {{ issue.description }}</span>
                            <span class="badge {% if issue.severity is containing('高') or issue.severity is containing('high') %}badge-high{% elif issue.severity is containing('中') or issue.severity is containing('medium') %}badge-medium{% else %}badge-low{% endif %}">
                                {{ issue.severity }}
                            </span>
                        </div>
                        <p style="color:var(--text-muted);font-size:13px;">📍 {{ issue.location }}</p>
                        <p style="margin-top:6px;"><strong>影响:</strong> {{ issue.impact }}</p>
                        <div class="recommendation">
                            <strong>💡 修复建议:</strong><br>
                            {{ issue.recommendation | replace(from="\n", to="<br>") | safe }}
                        </div>
                        {% if issue.best_practice %}<p style="margin-top:8px;font-size:13px;color:var(--text-muted);">📚 最佳实践: {{ issue.best_practice }}</p>{% endif %}
                    </div>
                    {% endfor %}
                </div>
                {% endif %}

                {% if analysis.coding_standard_issues | length == 0 and analysis.performance_issues | length == 0 and analysis.security_vulnerabilities | length == 0 %}
                <div class="card" style="text-align:center;padding:48px;">
                    <div style="font-size:48px;margin-bottom:16px;">🎉</div>
                    <h2>未发现任何问题</h2>
                    <p style="color:var(--text-muted);">静态代码分析通过，代码质量优秀！</p>
                </div>
                {% endif %}
            </div>

            <!-- ====== 覆盖率面板 ====== -->
            <div id="coverage" class="tab">
                <div class="card">
                    <h2>📈 覆盖率概览</h2>
                    <p style="color:var(--text-muted);margin-bottom:16px;">覆盖率数据由闭环引擎在每次迭代后自动采集</p>
                    <div class="dashboard" style="margin-bottom:0;">
                        <div class="metric-card primary">
                            <div class="metric-value">{{ test_results | length }}</div>
                            <div class="metric-label">总测试用例</div>
                        </div>
                        <div class="metric-card info">
                            <div class="metric-value">—</div>
                            <div class="metric-label">迭代次数</div>
                        </div>
                    </div>
                    <p style="margin-top:16px;color:var(--text-muted);font-size:13px;">
                        💡 提示：配置测试命令和覆盖率报告路径后，Agent 将自动执行测试并追踪覆盖率变化。
                    </p>
                </div>
            </div>

            <!-- 页脚 -->
            <div style="text-align:center;padding:24px;color:var(--text-muted);font-size:13px;">
                Generated by <strong>UnitestAgent</strong> — AI-Powered Unit Test Generation & Static Code Analysis
            </div>
        </div>

        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>
        <script>
            function showTab(tabId, el) {
                document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
                document.querySelectorAll('.tabs a').forEach(a => a.classList.remove('active'));
                document.getElementById(tabId).classList.add('active');
                if (el) el.classList.add('active');
                return false;
            }
        </script>
    </body>
    </html>
    "###;

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

    // 添加生成合并报告的函数
    pub async fn generate_combined_report(
        handle: AppHandle,
        unitest_id: String,
        test_results: &Vec<TestDetails>,
        analysis: &CodeAnalysisResult,
    ) -> Result<String, Box<dyn std::error::Error>> {

        let test_results_en: Vec<TestDetailsEn> =
        test_results.iter().map(|detail| detail.to_english()).collect();

        let analysisEn = CodeAnalysisResultEn {
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

        let mut tera: Tera = Tera::default();
        if let Err(e) = tera.add_raw_template("report", Self::COMBINED_TEMPLATE) {
            println!("Error adding raw template: {:?}", e);
            return Err(Box::new(e));
        }
        
        let mut context = Context::new();
        context.insert("test_results", &test_results_en);
        context.insert("analysis", &analysisEn);
        
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
        
        // 确保报告目录存在
        fs::create_dir_all(&reports_dir)?;
        
        // 创建报告文件名（使用时间戳确保唯一性）
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let file_name: String = format!("combined_report_{}.html", unitest_id);
        print!("file_name===={}", file_name);
        let report_path = reports_dir.join(&file_name);
        
        // 写入文件
        fs::write(&report_path, html_content)?;
        
        // 返回生成的报告文件路径
        Ok(report_path.to_string_lossy().to_string())
    }
}

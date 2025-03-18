use serde::Serialize;
use std::fs;
use tera::{Context, Tera};
// use crate::unitest_agent_test_generator::{TestDetails, TestDetailsEn};
use crate::app::uagent::{
    unitest_agent_test_generator::{TestDetails, TestDetailsEn},
};

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

    pub async fn generate_report(
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

        print!("======context:===== {:?}", context);

        let html_content = match tera.render("report", &context) {
            Ok(content) => content,
            Err(e) => {
                println!("模板渲染错误: {:?}", e);
                return Err(Box::new(e));
            }
        };

        println!("======html_content:===== {:?}", html_content);
        println!("======file_path:===== {:?}", file_path);
        
        fs::write(file_path, html_content)?;
        Ok(())
    }

    pub fn generate_report_tests(
        results: &Vec<TestResultsoutput>,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut tera = Tera::default();
        tera.add_raw_template("report", Self::HTML_TEMPLATE)?;
        let mut context: Context = Context::new();
        context.insert("results", &results);
        println!("======context:===== {:?}", context);
        // let html_content: String = tera.render("report", &context)?;
        let html_content = match tera.render("report", &context) {
            Ok(content) => content,
            Err(e) => {
                println!("模板渲染错误: {:?}", e);
                return Err(Box::new(e));
            }
        };
        let resource_dir = std::env::current_dir().unwrap().join("resources");
        std::fs::create_dir_all(&resource_dir)?;
        let resource_path = resource_dir.join(file_path);
        

        fs::write(&resource_path, html_content)?;
        opener::open(resource_path)?;
        Ok(())
    }
}

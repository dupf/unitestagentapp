use std::fs::File;
use std::io::Write;
use serde::Serialize;
use tera::Tera;

#[derive(Serialize)]
pub struct TestResult {
    测试用例编号: String,
    测试技术: String,
    测试用例描述: String,
    测试代码: String,
    全局变量: String,
    初始化代码: String,
    桩函数: String,
    输入: String,
    预期输出: String,
    实际输出: String,
    结论: String,
}

pub struct ReportGenerator;

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
            /* ... (CSS styles remain the same) ... */
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
                <td>{{ result.测试用例编号 }}</td>
                <td>{{ result.测试技术 }}</td>
                <td>{{ result.测试用例描述 }}</td>
                <td>{{ result.测试代码 }}</td>
                <td>{{ result.全局变量 }}</td>
                <td>{{ result.初始化代码 }}</td>
                <td>{{ result.桩函数 }}</td>
                <td>{{ result.输入 }}</td>
                <td>{{ result.预期输出 }}</td>
                <td>{{ result.实际输出 }}</td>
                <td>{{ result.结论 }}</td>
            </tr>
            {% endfor %}
        </table>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.23.0/prism.min.js"></script>
    </body>
    </html>
    "#;
    pub fn generate_report(results: &[TestResult], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Remove the debug print statement as TestResult doesn't implement Debug
        // println!("results: {:?}", results);

        let mut tera = Tera::default();
        tera.add_raw_template("report", Self::HTML_TEMPLATE)?;

        let mut context = tera::Context::new();
        context.insert("results", results);

        let html_content = tera.render("report", &context)?;

        let mut file = File::create(file_path)?;
        file.write_all(html_content.as_bytes())?;

        Ok(())
    }
}
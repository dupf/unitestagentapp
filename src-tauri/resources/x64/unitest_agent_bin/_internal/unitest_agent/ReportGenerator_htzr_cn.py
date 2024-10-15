from jinja2 import Template
import sys
import os
# sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

class ReportGenerator_htzr_cn:
    # Enhanced HTML template with additional styling
    HTML_TEMPLATE = """
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
    """

    @classmethod
    def generate_report(cls, results, file_path):
        """
        Renders the HTML report with given results and writes to a file.

        :param results: List of dictionaries with test results.
        :param file_path: Path to the HTML file where the report will be written.
        """

        template = Template(cls.HTML_TEMPLATE)
        html_content = template.render(results=results)

        with open(file_path, "w") as file:
            file.write(html_content)
        
        


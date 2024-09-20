use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::sync::Once;

pub struct CustomLogger;

static INIT: Once = Once::new();

impl CustomLogger {
    pub fn get_logger(name: &str) -> Result<(), log::SetLoggerError> {
        let _ = name;
        INIT.call_once(|| {
            let log_file_path = "run.log";

            // 文件处理器
            let file_appender = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
                .build(log_file_path)
                .unwrap();

            // 控制台处理器
            let console_appender = ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{l} - {m}{n}")))
                .build();

            // 配置
            let config = Config::builder()
                .appender(Appender::builder().build("file", Box::new(file_appender)))
                .appender(Appender::builder().build("console", Box::new(console_appender)))
                .build(
                    Root::builder()
                        .appender("file")
                        .appender("console")
                        .build(LevelFilter::Debug),
                )
                .unwrap();

            // 初始化日志系统
            log4rs::init_config(config).unwrap();
        });

        log::set_max_level(LevelFilter::Debug);
        Ok(())
    }
}

// 使用示例
fn main() {
    CustomLogger::get_logger("my_logger").unwrap();

    log::debug!("This is a debug message");
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");
}
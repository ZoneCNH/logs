pub use log::{info, error, warn, debug, trace};
pub use chrono::Local;

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // 确保日志目录存在
    if let Err(e) = std::fs::create_dir_all("logs") {
        eprintln!("Failed to create logs directory: {}", e);
        return Ok(());
    }
    
    // 动态生成带日期的日志文件名
    let now = Local::now();
    let log_file = format!("logs/app_{}.log", now.format("%Y-%m-%d"));
    println!("Log file path: {}", log_file);
    
    // 手动创建配置
    let config = log4rs::config::Config::builder()
        .appender(
            log4rs::config::Appender::builder()
                .build("stdout", Box::new(log4rs::append::console::ConsoleAppender::builder().build()))
        )
        .appender(
            log4rs::config::Appender::builder()
                .build("file", Box::new(log4rs::append::file::FileAppender::builder()
                    .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{d} {l} {t} - {m}{n}")))
                    .build(log_file)
                    .unwrap()))
        )
        .build(
            log4rs::config::Root::builder()
                .appender("stdout")
                .appender("file")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    if let Err(e) = log4rs::init_config(config) {
        eprintln!("Failed to initialize log4rs: {}", e);
    }

    Ok(())
}

pub fn test_logging() {
    let i = 0;
    trace!("This is a trace message {}", i);
    debug!("This is a debug message {}", i);
    info!("This is an info message {}", i);
    warn!("This is a warning message {}", i);
    error!("This is an error message {}", i);
}

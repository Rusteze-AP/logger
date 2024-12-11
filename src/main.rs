mod logger;
use logger::LogLevel;
use std::{thread, time::Duration};

fn main() {
    
    let logger = logger::Logger::new(LogLevel::Debug as u8 | LogLevel::Error as u8 | LogLevel::Info as u8, true, "main".to_string());
    loop {
        //waiit for a second
        logger.log_error("This is a error");
        thread::sleep(Duration::from_millis(1000));
        logger.log_debug("This is a debug message");
        thread::sleep(Duration::from_millis(1000));
        logger.log_info("This is a infoooo");
        thread::sleep(Duration::from_millis(1000));
    }
}


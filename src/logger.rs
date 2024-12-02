
pub struct Logger {
    displayable: bool,
    module: String,
}


impl Logger {
    pub fn new(displayable: bool, module: String) -> Logger {
        Logger {
            displayable,
            module,
        }
    }
    
    pub fn set_displayable(&mut self, displayable: bool) {
        self.displayable = displayable;
    }
    
    pub fn log_debug(&self, message: &str) {
        if self.displayable {
            println!("\x1b[32m[DEBUG][{}]    ||{}\x1b[0m",self.module,  message);
        }
    }
    
    pub fn log_info(&self, message: &str) {
        if self.displayable {
            println!("\x1b[34m[INFO][{}]    ||{}\x1b[0m",self.module,  message);
        }
    }
    
    pub fn log_warn(&self, message: &str) {
        if self.displayable {
            println!("\x1b[33m[WARN][{}]    ||{}\x1b[0m",self.module,  message);
        }
    }
    
    pub fn log_error(&self, message: &str) {
        if self.displayable {
            println!("\x1b[31m[ERR][{}]     ||{}\x1b[0m",self.module,  message);
        }
    }
}
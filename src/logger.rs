use chrono::Local;
use futures::SinkExt;
use futures::StreamExt;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::broadcast;
use warp::Filter;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    None = 0b0000,  // 0
    Debug = 0b0001, // 1
    Warn = 0b0010,  // 2
    Error = 0b0100, // 4
    Info = 0b1000,  // 8
    All = 0b1111,   // 15
}

pub struct Logger {
    displayable: u8,
    web_socket: bool,
    module: String,
    log_sender: broadcast::Sender<String>,
}

impl Logger {
    pub fn new(displayable: u8, web_socket: bool, module: String) -> Logger {
        let (log_sender, _) = broadcast::channel(100);

        Logger {
            displayable,
            web_socket: false,
            module,
            log_sender,
        }
    }

    pub fn set_displayable(&mut self, displayable: u8) {
        self.displayable = displayable;
    }

    pub fn add_displayable_flag(&mut self, flag: LogLevel) {
        self.displayable |= flag as u8;
    }

    pub fn remove_displayable_flag(&mut self, flag: LogLevel) {
        self.displayable &= !(flag as u8);
    }

    pub fn init_web_socket(&mut self) {
        let thread_log_sender = self.log_sender.clone();
        self.web_socket = true;
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let ws_route = warp::path("ws")
                    .and(warp::ws())
                    .and(warp::any().map(move || thread_log_sender.clone()))
                    .map(|ws: warp::ws::Ws, sender| {
                        ws.on_upgrade(move |socket| handle_connection(socket, sender))
                    });

                println!("WebSocket server running on ws://127.0.0.1:3030");

                // Run the warp server.
                warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
            });
        });
    }

    fn send_to_socket(&self, formatted_message: String) {
        if self.web_socket {
            let _ = self.log_sender.send(formatted_message);
        }
    }

    pub fn log_debug(&self, message: &str) {
        let time = Local::now().format("%H:%M:%S:%3f").to_string();
        let formatted_message = format!("[{}][DEBUG][{}] {}", time, self.module, message);

        if self.displayable & LogLevel::Debug as u8 != 0 {
            println!("\x1b[32m{}\x1b[0m", formatted_message);
        }

        self.send_to_socket(formatted_message);
    }

    pub fn log_info(&self, message: &str) {
        let time = Local::now().format("%H:%M:%S:%3f").to_string();
        let formatted_message = format!("[{}][INFO][{}] {}", time, self.module, message);

        if self.displayable & LogLevel::Info as u8 != 0 {
            println!("\x1b[34m{}\x1b[0m", formatted_message);
        }

        self.send_to_socket(formatted_message);
    }

    pub fn log_warn(&self, message: &str) {
        let time = Local::now().format("%H:%M:%S:%3f").to_string();
        let formatted_message = format!("[{}][WARN][{}] {}", time, self.module, message);

        if self.displayable & LogLevel::Warn as u8 != 0 {
            println!("\x1b[33m{}\x1b[0m", formatted_message);
        }

        self.send_to_socket(formatted_message);
    }

    pub fn log_error(&self, message: &str) {
        let time = Local::now().format("%H:%M:%S:%3f").to_string();
        let formatted_message = format!("[{}][ERR][{}] {}", time, self.module, message);

        if self.displayable & LogLevel::Error as u8 != 0 {
            println!("\x1b[31m{}\x1b[0m", formatted_message);
        }

        self.send_to_socket(formatted_message);
    }
}

async fn handle_connection(ws: warp::ws::WebSocket, sender: broadcast::Sender<String>) {
    let (mut ws_tx, _) = ws.split();
    let mut rx = sender.subscribe();

    while let Ok(message) = rx.recv().await {
        if ws_tx.send(warp::ws::Message::text(message)).await.is_err() {
            break;
        }
    }
}

use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub struct Logger;

impl Logger {
    pub fn init() {
        let args: Vec<String> = env::args().collect();
        let level = args
            .windows(2)
            .find(|w| w[0] == "--log-level")
            .and_then(|w| w.get(1))
            .map(|lvl| match lvl.as_str() {
                "error" => Level::ERROR,
                "warn"  => Level::WARN,
                "debug" => Level::DEBUG,
                "trace" => Level::TRACE,
                _       => Level::INFO,
            })
            .unwrap_or(Level::INFO);

        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

        info!("Logger initialized with level: {:?}", level);
    }
}

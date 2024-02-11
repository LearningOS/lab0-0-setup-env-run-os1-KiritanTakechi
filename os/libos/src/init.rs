use crate::logger::Logger;

static LOGGER: Logger = Logger;
pub struct InitConfig;

impl InitConfig {
    pub fn init() {
        let env2level = || match option_env!("LOG").unwrap_or("TRACE") {
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "WARN" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        };
        
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(env2level());
    }
}
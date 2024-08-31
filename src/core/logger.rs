pub fn init_logging() {
    let config_path = std::env::var("RUSTLE_CONFIG").unwrap_or_else(|_| "config/log4rs.yaml".to_string());
    if let Err(e) = log4rs::init_file(config_path, Default::default()) {
        eprintln!("Error initializing logger: {}", e);
    }
}

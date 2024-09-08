mod config;

use config::Config;

fn main() {
    match Config::new() {
        Ok(config) => {
            println!("Server config: {:?}", config.server);
            println!("Database config: {:?}", config.database);
            println!("Logging config: {:?}", config.logging);
        },
        Err(e) => eprintln!("Failed to load config: {}", e),
    }
}

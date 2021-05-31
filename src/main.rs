use std::io::Read;

mod config;
mod serde_error;

use config::Config;
use serde_error::SerdeError;

fn main() -> Result<(), anyhow::Error> {
    if let Err(err) = test_yaml() {
        print!("test_yaml");
        println!("{}", err)
    }

    if let Err(err) = test_json() {
        print!("test_json");
        println!("{}", err)
    }

    Ok(())
}

fn test_yaml() -> Result<(), anyhow::Error> {
    let mut reader = std::io::BufReader::new(std::fs::File::open("config.yaml")?);
    let mut config_str = String::new();

    reader.read_to_string(&mut config_str)?;
    let _config: Config = match serde_yaml::from_str(&config_str) {
        Ok(c) => c,
        Err(err) => return Err(SerdeError::new(config_str, err)?.into()),
    };

    Ok(())
}

fn test_json() -> Result<(), anyhow::Error> {
    let mut reader = std::io::BufReader::new(std::fs::File::open("config_pretty.json")?);
    let mut config_str = String::new();

    reader.read_to_string(&mut config_str)?;
    let _config: Config = match serde_json::from_str(&config_str) {
        Ok(c) => c,
        Err(err) => return Err(SerdeError::new(config_str, err)?.into()),
    };

    Ok(())
}

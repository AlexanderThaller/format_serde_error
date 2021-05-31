use anyhow::bail;
use std::io::Read;

mod config;

use crate::SerdeError;
use config::Config;

#[test]
fn test_yaml() -> Result<(), anyhow::Error> {
    let mut reader = std::io::BufReader::new(std::fs::File::open("../../resources/config.yaml")?);
    let mut config_str = String::new();

    reader.read_to_string(&mut config_str)?;
    let _config: Config = match serde_yaml::from_str(&config_str) {
        Ok(c) => c,
        Err(err) => return Err(SerdeError::new(config_str, err)?.into()),
    };

    Ok(())
}

#[test]
fn test_json() -> Result<(), anyhow::Error> {
    let mut reader =
        std::io::BufReader::new(std::fs::File::open("../../resources/config_pretty.json")?);
    let mut config_str = String::new();

    let expected = String::new();

    reader.read_to_string(&mut config_str)?;
    let got = match serde_json::from_str::<Config>(&config_str) {
        Ok(c) => bail!("expecting error got a ok"),
        Err(err) => format!("{}", SerdeError::new(config_str, err)?),
    };

    assert_eq!(expected, got);

    Ok(())
}

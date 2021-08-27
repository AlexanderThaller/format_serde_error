use format_serde_error::SerdeError;

#[derive(Debug, serde::Deserialize)]
struct Config {
    values: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let config_str = r#"values = [
    "first",
    "second",
    third =
]"#;

    let config = toml::from_str::<Config>(config_str)
        .map_err(|err| SerdeError::new(config_str.to_string(), err))?;

    dbg!(config);

    Ok(())
}

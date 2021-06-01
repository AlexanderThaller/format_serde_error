use format_serde_error::SerdeError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    values: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let config_str = r#"{"values":["asd","asd2",{"invalid":"dont"},"asd3","asd4"]}"#;

    let config = serde_json::from_str::<Config>(config_str)
        .map_err(|err| SerdeError::new(config_str.to_string(), err))?;

    dbg!(config);

    Ok(())
}

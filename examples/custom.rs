use anyhow::anyhow;

use format_serde_error::SerdeError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    values: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let config_str = "values:
  - 'first'
  - 'second'
  - third:";

    let err = anyhow!("values[2]: invalid type: map, expected a string at line 4 column 9");
    Err(SerdeError::new(config_str.to_string(), (err.into(), Some(4), Some(9))).into())
}

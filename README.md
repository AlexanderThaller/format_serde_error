# format_serde_error

[![Build Status](https://github.com/AlexanderThaller/format_serde_error/workflows/Rust/badge.svg?branch=main)](https://github.com/AlexanderThaller/format_serde_error/actions?query=workflow%3ARusteain)
[![crates.io](https://img.shields.io/crates/v/format_serde_error.svg)](https://crates.io/crates/format_serde_error)

!["example output"](resources/example.png)

Format serde errors in a way to make it obvious where the error in the source file was.

Example:

```rust
use format_serde_error::SerdeError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    values: Vec<String>,
}

fn parse_config() -> Result<Config, anyhow::Error> {
  let config_str = "values:
  - 'first'
  - 'second'
  - third:";

  let config = serde_yaml::from_str::<Config>(config_str)
    .map_err(|err| SerdeError::new(config_str.to_string(), err))?;

  Ok(config)
}
```

The output will be:

```
Error:
   | values:
   |   - 'first'
   |   - 'second'
 4 |   - third:
   |           ^ values[2]: invalid type: map, expected a string at line 4 column 10
```

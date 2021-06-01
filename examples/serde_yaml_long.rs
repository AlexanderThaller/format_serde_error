use format_serde_error::SerdeError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    values: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let config_str = "values:
  - 'first'
  - 'second'
  - third: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur vestibulum tempor dolor, aliquam finibus odio lacinia ut. Vivamus fermentum odio ac nibh efficitur efficitur. Morbi id lorem tortor. Duis non quam pellentesque, varius dui ut, accumsan ante. Etiam nec tortor sit amet ipsum suscipit consectetur in eu felis. Vivamus lacus odio, tincidunt ac elit vel, varius ultricies est. In blandit tincidunt interdum.";

    let config = serde_yaml::from_str::<Config>(config_str)
        .map_err(|err| SerdeError::new(config_str.to_string(), err))?;

    dbg!(config);

    Ok(())
}

mod config;

use crate::SerdeError;
use config::Config;

mod yaml {
    use anyhow::bail;
    use colored::Colorize;

    use super::{
        Config,
        SerdeError,
    };

    fn run_yaml(config_str: &str) -> Result<String, anyhow::Error> {
        match serde_yaml::from_str::<Config>(config_str) {
            Ok(_) => bail!("expecting error got a ok"),
            Err(err) => {
                dbg!(&err);

                Ok(format!(
                    "{}",
                    dbg!(SerdeError::new(config_str.to_string(), err)?)
                ))
            }
        }
    }

    #[test]
    fn empty_config_file() -> Result<(), anyhow::Error> {
        let input = "";
        let expected = format!("{}\n", "EOF while parsing a value".red().bold());
        let got = run_yaml(input)?;

        print!("{}", expected);

        assert_eq!(expected, got);

        Ok(())
    }
}

mod json {
    use anyhow::bail;
    use colored::Colorize;

    use super::{
        Config,
        SerdeError,
    };

    fn run_json(config_str: &str) -> Result<String, anyhow::Error> {
        match serde_json::from_str::<Config>(config_str) {
            Ok(_) => bail!("expecting error got a ok"),
            Err(err) => {
                dbg!(&err);

                Ok(format!("{}", SerdeError::new(config_str.to_string(), err)?))
            }
        }
    }

    #[test]
    fn empty_config_file() -> Result<(), anyhow::Error> {
        let input = "";
        let expected = format!(
            "{}\n",
            "EOF while parsing a value at line 1 column 0".red().bold()
        );
        let got = run_json(input)?;

        assert_eq!(expected, got);

        Ok(())
    }
}

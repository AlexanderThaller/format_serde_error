use colored::{
    ColoredString,
    Colorize,
};

mod config;

use crate::SerdeError;
use config::Config;

fn separator() -> ColoredString {
    " | ".blue().bold()
}

fn init() {
    crate::never_color();
}

#[cfg(feature = "serde_yaml")]
mod yaml {
    use anyhow::bail;
    use colored::Colorize;
    use pretty_assertions::assert_eq;

    use super::{
        Config,
        SerdeError,
    };

    fn run_yaml(config_str: &str) -> Result<String, anyhow::Error> {
        match serde_yaml::from_str::<Config>(config_str) {
            Ok(_) => bail!("expecting error got a ok"),
            Err(err) => Ok(format!("{}", SerdeError::new(config_str.to_string(), err))),
        }
    }

    #[test]
    fn empty_config_file() -> Result<(), anyhow::Error> {
        super::init();

        let input = "";
        let expected = format!("{}\n", "EOF while parsing a value".red().bold());
        let got = run_yaml(input)?;

        print!("{}", expected);

        assert_eq!(expected, got);

        Ok(())
    }

    #[test]
    fn example_config_file() -> Result<(), anyhow::Error> {
        super::init();
        let separator = super::separator();

        let input = include_str!("../../resources/config.yaml");

        let mut expected = String::new();
        expected.push_str("\n");

        expected.push_str(&format!("    {}{}\n", separator, r#"- 'asd110'"#.yellow()));
        expected.push_str(&format!("    {}{}\n", separator, r#"- 'asd111'"#.yellow()));
        expected.push_str(&format!("    {}{}\n", separator, r#"- 'asd112'"#.yellow()));

        expected.push_str(&format!(
            " {}{}{}\n",
            "114".blue().bold(),
            separator,
            "- invalid: 'dont'",
        ));

        expected.push_str(&format!(
            "    {}{}\n",
            separator,
            "          ^ values[112]: invalid type: map, expected a string at line 114 column 12"
                .red()
                .bold()
        ));

        expected.push_str(&format!("    {}{}\n", separator, r#"- 'asd113'"#.yellow()));
        expected.push_str(&format!("    {}{}\n", separator, r#"- 'asd114'"#.yellow()));
        expected.push_str(&format!("    {}{}\n", separator, r#"- 'asd115'"#.yellow()));

        let got = run_yaml(input)?;

        println!("got:\n{}", got);
        println!("expected:\n{}", expected);

        assert_eq!(expected, got);

        Ok(())
    }
}

#[cfg(feature = "serde_json")]
mod json {
    use anyhow::bail;
    use colored::Colorize;
    use pretty_assertions::assert_eq;

    use super::{
        Config,
        SerdeError,
    };

    fn run_json(config_str: &str) -> Result<String, anyhow::Error> {
        match serde_json::from_str::<Config>(config_str) {
            Ok(_) => bail!("expecting error got a ok"),
            Err(err) => Ok(format!("{}", SerdeError::new(config_str.to_string(), err))),
        }
    }

    #[test]
    fn empty_config_file() -> Result<(), anyhow::Error> {
        super::init();

        let input = "";
        let expected = format!(
            "{}\n",
            "EOF while parsing a value at line 1 column 0".red().bold(),
        );
        let got = run_json(input)?;

        println!("{}", expected);
        println!("{}", got);

        assert_eq!(expected, got);

        Ok(())
    }

    #[test]
    fn empty_config_file_only_map() -> Result<(), anyhow::Error> {
        super::init();
        let separator = super::separator();

        let input = "{}";

        let mut expected = String::new();
        expected.push_str("\n");
        expected.push_str(&format!(" {}{}{}\n", "1".blue().bold(), separator, "{}",));
        expected.push_str(&format!(
            "  {} {}\n",
            separator,
            " ^ missing field `values` at line 1 column 2".red().bold(),
        ));

        let got = run_json(input)?;

        println!("{}", expected);
        println!("{}", got);

        assert_eq!(expected, got);

        Ok(())
    }

    #[test]
    fn unterminated_map() -> Result<(), anyhow::Error> {
        super::init();
        let separator = super::separator();

        let input = "{";

        let mut expected = String::new();
        expected.push_str("\n");
        expected.push_str(&format!(" {}{}{}\n", "1".blue().bold(), separator, "{",));
        expected.push_str(&format!(
            "  {} {}\n",
            separator,
            "^ EOF while parsing an object at line 1 column 1"
                .red()
                .bold(),
        ));

        let got = run_json(input)?;

        println!("{}", expected);
        println!("{}", got);

        assert_eq!(expected, got);

        Ok(())
    }

    #[test]
    fn example_config_file() -> Result<(), anyhow::Error> {
        super::init();

        let input = include_str!("../../resources/config_pretty.json");
        let separator = super::separator();

        let mut expected = String::new();
        expected.push_str("\n");

        expected.push_str(&format!("    {}{}\n", separator, r#""asd110","#.yellow()));
        expected.push_str(&format!("    {}{}\n", separator, r#""asd111","#.yellow()));
        expected.push_str(&format!("    {}{}\n", separator, r#""asd112","#.yellow()));

        expected.push_str(&format!(" {}{}{}\n", "115".blue().bold(), separator, "{",));

        expected.push_str(&format!(
            "    {}{}\n",
            separator,
            "^ invalid type: map, expected a string at line 115 column 4"
                .red()
                .bold()
        ));

        expected.push_str(&format!(
            "    {}{}\n",
            separator,
            r#"  "invalid": "dont""#.yellow()
        ));

        expected.push_str(&format!("    {}{}\n", separator, r#"},"#.yellow()));
        expected.push_str(&format!("    {}{}\n", separator, r#""asd113","#.yellow()));

        let got = run_json(input)?;

        println!("got:\n{}", got);
        println!("expected:\n{}", expected);

        assert_eq!(expected, got);

        Ok(())
    }
}

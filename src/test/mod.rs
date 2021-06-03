#[cfg(feature = "colored")]
use colored::{
    ColoredString,
    Colorize,
};

mod config;

use crate::SerdeError;
#[allow(unused_imports)]
use config::Config;

#[cfg(feature = "colored")]
fn separator() -> ColoredString {
    super::SEPARATOR.blue()
}

#[cfg(feature = "colored")]
fn ellipse() -> ColoredString {
    super::ELLIPSE.blue().bold()
}

fn init() {
    #[cfg(feature = "colored")]
    crate::never_color();
}

// TODO: Make tests that only use serde_yaml feature
#[cfg(all(feature = "serde_yaml", feature = "colored"))]
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

        print!("expected:{}", expected);

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

        println!("got:{}", got);
        println!("expected:{}", expected);

        assert_eq!(expected, got);

        Ok(())
    }
}

// TODO: Make tests that only use serde_json feature
#[cfg(all(feature = "serde_json", feature = "colored"))]
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

        println!("expected:{}", expected);
        println!("got:{}", got);

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
            "  {}{}\n",
            separator,
            "  ^ missing field `values` at line 1 column 2".red().bold(),
        ));

        let got = run_json(input)?;

        println!("expected:{}", expected);
        println!("got:{}", got);

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
            "  {}{}\n",
            separator,
            " ^ EOF while parsing an object at line 1 column 1"
                .red()
                .bold(),
        ));

        let got = run_json(input)?;

        println!("expected:{}", expected);
        println!("got:{}", got);

        assert_eq!(expected, got);

        Ok(())
    }

    #[test]
    fn example_config_file_pretty() -> Result<(), anyhow::Error> {
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

        println!("expected:{}", expected);
        println!("got:{}", got);

        assert_eq!(expected, got);

        Ok(())
    }

    #[test]
    fn example_config_file() -> Result<(), anyhow::Error> {
        super::init();

        let input = include_str!("../../resources/config.json");
        let separator = super::separator();
        let ellipse = super::ellipse();

        let mut expected = String::new();
        expected.push_str("\n");

        expected.push_str(&format!(
            " {}{}{}{}{}\n",
            "1".blue().bold(),
            separator,
            ellipse,
            r#"09","asd110","asd111","asd112",{"invalid":"dont"},"asd113","a"#,
            ellipse
        ));

        expected.push_str(&format!(
            "  {}{}\n",
            separator,
            "                                  ^ invalid type: map, expected a string at line 1 \
             column 910"
                .red()
                .bold()
        ));

        let got = run_json(input)?;

        println!("expected:{}", expected);
        println!("got:{}", got);

        assert_eq!(expected, got);

        Ok(())
    }
}

mod context_long_line {
    use pretty_assertions::assert_eq;

    const SHORT_LINE: &str = "abc!def";
    const LONG_LINE: &str = "?orem ipsum dolor sit amet, consectetur adipiscing elit. Morbi \
                             luctus accumsan lorem, vulputate laci!nia tellus sodales sed. \
                             Phasellus libero ipsum, ornare quis ullamcorper sed, porttitor \
                             congue lorem. Phasellus turpis lectus, vestibulum sit amet ex in, \
                             dignissim rhoncus dolor.";

    /// Short line and we want the full line as context
    #[test]
    fn short_line_without_context() {
        let input = SHORT_LINE;
        let error_column = 4;
        let context_chars = 1000;
        let expected = input.to_string();
        let expected_char = '!';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        dbg!(new_error_column);

        assert_eq!(expected, got);
        assert_eq!(expected_char, got_char);
        assert!(!context_before);
        assert!(!context_after);
    }

    /// Short line and we want only part of the line as context
    #[test]
    fn short_line_using_context() {
        let input = SHORT_LINE;
        let error_column = 4;
        let context_chars = 2;
        let expected = "bc!de";
        let expected_char = '!';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        assert_eq!(context_chars * 2 + 1, got.len());
        assert_eq!(expected, got);
        assert_eq!(expected_char, got_char);
        assert!(context_before);
        assert!(context_after);
    }

    /// Long line and we want only part of the line as context
    #[test]
    fn long_line_error_beginning() {
        let input = LONG_LINE;
        let error_column = 1;
        let context_chars = 20;
        let expected = "?orem ipsum dolor sit amet, consectetur a";
        let expected_char = '?';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        assert_eq!(context_chars * 2 + 1, got.len());
        assert_eq!(expected, got);
        assert_eq!(expected_char, got_char);
        assert!(!context_before);
        assert!(context_after);
    }

    /// Long line and we want only part of the line as context
    #[test]
    fn long_line_using_context() {
        let input = LONG_LINE;
        let error_column = 101;
        let context_chars = 20;
        let expected = "orem, vulputate laci!nia tellus sodales s";
        let expected_char = '!';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        assert_eq!(context_chars * 2 + 1, got.len());
        assert_eq!(expected, got);
        assert_eq!(expected_char, got_char);
        assert!(context_before);
        assert!(context_after);
    }

    /// More structured line that makes debugging easier
    #[test]
    fn structured_line() {
        let input = "abcdefghij0123456789!0123456789klmnopqrst";
        let error_column = 21;
        let context_chars = 10;
        let expected = "0123456789!0123456789";
        let expected_char = '!';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        assert_eq!(context_chars * 2 + 1, got.len());
        assert_eq!(expected, got);
        assert_eq!(expected_char, got_char);
        assert!(context_before);
        assert!(context_after);
    }

    /// Test for the error being the last char in the line
    #[test]
    fn last_char_is_error() {
        let input = "abcdefghij01234567890123456789klmnopqrst!";
        let error_column = 41;
        let context_chars = 10;
        let expected = "klmnopqrst!";
        let expected_char = '!';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        assert_eq!(11, got.len());
        assert_eq!(expected, got);
        assert_eq!(expected_char, got_char);
        assert!(context_before);
        assert!(!context_after);
    }

    /// Test for unicode compatibility
    #[test]
    fn unicode_string() {
        let input = "\u{20ac}123456789!\u{20ac}123456789";
        let error_column = 11;
        let context_chars = 5;
        let expected = "56789!\u{20ac}1234";
        let expected_char = '!';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        // 13 instead of 11 because len for a string gives back the amount of codepoints
        // not the amount of characters
        assert_eq!(13, got.len());
        assert_eq!(expected_char, got_char);
        assert_eq!(expected, got);
        assert!(context_before);
        assert!(context_after);
    }

    /// Test for graphemes compatibility
    #[cfg(feature = "graphemes_support")]
    #[test]
    fn graphemes_string() {
        let input = "a\u{310}e\u{301}o\u{308}\u{332}3456789!a\u{310}e\u{301}o\u{308}\u{332}3456789";
        let error_column = 11;
        let context_chars = 5;
        let expected = "56789!a\u{310}e\u{301}o\u{308}\u{332}34";
        let expected_char = '!';

        let (got, new_error_column, context_before, context_after) =
            super::SerdeError::context_long_line(input, error_column, context_chars);
        let got_char = got.chars().nth(new_error_column - 1).unwrap_or_default();

        // 19 instead of 11 because len for a string gives back the amount of codepoints
        // not the amount of characters
        assert_eq!(19, got.len());
        assert_eq!(expected_char, got_char);
        assert_eq!(expected, got);
        assert!(context_before);
        assert!(context_after);
    }
}

mod custom {
    use pretty_assertions::assert_eq;

    #[test]
    fn custom_error() {
        super::init();

        let config_str = "this is just a config file\nthe error is here: !";
        let line = 2;
        let column = 19;
        let err = format!("Found an error at line {}, column {}", line, column);

        let mut expected = String::from("\n");
        expected.push_str("   | this is just a config file\n");
        expected.push_str(" 2 | the error is here: !\n");
        expected.push_str("   |                    ^ Found an error at line 2, column 19\n");

        let got = format!(
            "{}",
            super::SerdeError::new(
                config_str.to_string(),
                (err.into(), Some(line), Some(column))
            )
        );

        println!("got:\n{}", got);
        println!("expected:\n{}", expected);

        assert_eq!(expected, got);
    }

    #[test]
    fn custom_error_long_line() {
        super::init();

        let config_str = "this is just a config file\nthe error that is somewhere in this line \
                          will be found somewhere after here maybe we can find it here: !, it \
                          could also be somewhere else maybe we will find that out someda, it \
                          could also be somewhere else maybe we will find that out someday";
        let line = 2;
        let column = 103;
        let err = format!("Found an error at line {}, column {}", line, column);

        let mut expected = String::from("\n");
        expected.push_str("   | this is just a config file\n");
        expected
            .push_str(" 2 | ...ere maybe we can find it here: !, it could also be somewhere ...\n");
        expected.push_str(
            "   |                                   ^ Found an error at line 2, column 103\n",
        );

        let got = format!(
            "{}",
            super::SerdeError::new(
                config_str.to_string(),
                (err.into(), Some(line), Some(column))
            )
        );

        println!("got:\n{}", got);
        println!("expected:\n{}", expected);

        assert_eq!(expected, got);
    }
}

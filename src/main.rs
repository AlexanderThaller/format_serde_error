use anyhow::anyhow;
use colored::*;
use prettytable::{
    cell,
    format::FormatBuilder,
    row,
    Table,
};
use std::{
    fmt,
    io::Read,
};

mod config;

use config::Config;

fn main() -> Result<(), anyhow::Error> {
    if let Err(err) = test_yaml() {
        println!("{}", err)
    }

    if let Err(err) = test_json() {
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
    let mut reader = std::io::BufReader::new(std::fs::File::open("config.json")?);
    let mut config_str = String::new();

    reader.read_to_string(&mut config_str)?;
    let _config: Config = match serde_json::from_str(&config_str) {
        Ok(c) => c,
        Err(err) => return Err(SerdeError::new(config_str, err)?.into()),
    };

    Ok(())
}

#[derive(Debug)]
struct SerdeError {
    input: String,
    message: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
enum ErrorTypes {
    Yaml(serde_yaml::Error),
    Json(serde_json::Error),
}

impl std::error::Error for SerdeError {}

impl fmt::Display for SerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl From<serde_yaml::Error> for ErrorTypes {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Yaml(err)
    }
}

impl From<serde_json::Error> for ErrorTypes {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl SerdeError {
    fn new(input: String, err: impl Into<ErrorTypes>) -> Result<SerdeError, anyhow::Error> {
        let error = err.into();

        let (message, line, column) = match error {
            ErrorTypes::Yaml(ref e) => {
                let location = e
                    .location()
                    .ok_or_else(|| anyhow!("no location found in error"))?;

                (e.to_string(), location.line(), location.column())
            }

            ErrorTypes::Json(ref e) => (e.to_string(), e.line(), e.column()),
        };

        Ok(Self {
            input,
            message,
            line,
            column,
        })
    }

    fn format(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Amount of lines to show before and after the error line
        let context = 3;

        // Skip until we are amount of context lines before the error line (context)
        // plus the line with the error ( + 1)
        // Saturating sub if the error is in the first line we can't take more context
        let skip = usize::saturating_sub(self.line, context + 1);

        // Take lines before and after (context * 2) plus the line with the error ( + 1)
        let take = context * 2 + 1;

        // To reduce the amount of space text takes we want to remove unnecessary
        // whitespace in front of the text.
        // Find the line with the least amount of whitespace in front and use
        // that to remove the whitespace later.
        // We basically want to find the least indented line.
        // We cant just use trim as that would remove all whitespace and remove all
        // indentation.
        let whitespace_count = self
            .input
            .lines()
            .skip(skip)
            .take(take)
            .map(|line| line.chars().take_while(|s| s.is_whitespace()).count())
            .min()
            .unwrap_or_default();

        let mut table = Table::new();
        // No padding, or other formatting
        table.set_format(FormatBuilder::new().build());

        let separator = " | ".blue().bold();

        for (line, text) in self.input
        .lines()
        .into_iter()
        .enumerate()
        .skip(skip)
        .take(take)
        // Make the index start at 1 makes it nicer to work with
        // Also remove unnecessary whitespace in front of text
        .map(|(index, text)| (index + 1, text.chars().skip(whitespace_count).collect::<String>()))
        {
            if line != self.line {
                // Print context lines
                table.add_row(row!["", separator, text.yellow(),]);
            } else {
                // Print error line
                table.add_row(row![
                    format!(" {}", line).to_string().blue().bold(),
                    separator,
                    text,
                ]);

                // Print error information
                table.add_row(row![
                    "",
                    separator,
                    format!(
                        "{: >column$}^ {}",
                        "",
                        self.message,
                        column = self.column - whitespace_count
                    )
                    .red()
                    .bold(),
                ]);
            }
        }

        // Want to avoid printing when we are not at the beginning of the line. For
        // example anyhow will write 'Error:' in front of the output before
        // printing the table
        writeln!(f)?;
        write!(f, "{}", table)?;

        Ok(())
    }
}

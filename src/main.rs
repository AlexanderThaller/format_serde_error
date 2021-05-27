use colored::*;
use prettytable::{
    cell,
    format::FormatBuilder,
    row,
    Table,
};

mod config;

use config::Config;

fn main() -> Result<(), anyhow::Error> {
    let reader = std::io::BufReader::new(std::fs::File::open("config.yaml")?);
    let config: serde_yaml::Value = serde_yaml::from_reader(reader)?;

    let json_str = serde_json::to_string_pretty(&config)?;

    let _config_from_json: Config = match serde_json::from_str(&json_str) {
        Ok(c) => c,
        Err(err) => return print_err(&json_str, err),
    };

    Ok(())
}

fn print_err(json_str: &str, err: serde_json::Error) -> Result<(), anyhow::Error> {
    if !err.is_data() {
        return Err(err.into());
    }

    // Amount of lines to show before and after the error line
    let context = 3;

    // Skip until we are amount of context lines before the error line (context)
    // plus the line with the error ( + 1)
    // Saturating sub if the error is in the first line we can't take more context
    let skip = usize::saturating_sub(err.line(), context + 1);

    // Take lines before and after (context * 2) plus the line with the error ( + 1)
    let take = context * 2 + 1;

    // To reduce the amount of space text takes we want to remove unneccessary
    // whitespace in front of the text.
    // Find the line with the least amount of whitespace in front and use
    // that to remove the whitespaces later.
    // We basically want to find the least indented line.
    // We cant just use trim as that would remove all whitespace and remove all
    // indentation.
    let whitespaces = json_str
        .lines()
        .skip(skip)
        .take(take)
        .map(|line| line.chars().take_while(|s| s.is_whitespace()).count())
        .min()
        .unwrap_or_default();

    let mut table = Table::new();
    // No padding, or other formatting
    table.set_format(FormatBuilder::new().build());

    let sepperator = " | ".blue().bold();

    for (line, text) in json_str
        .lines()
        .into_iter()
        .enumerate()
        .skip(skip)
        .take(take)
        // Make the index start at 1 makes it nicer to work with
        // Also remove whitespaces used for formatting
        .map(|(index, line)| (index + 1, line.chars().skip(whitespaces).collect::<String>()))
    {
        if line != err.line() {
            // Print context lines
            table.add_row(row!["", sepperator, text.yellow(),]);
        } else {
            // Print error line
            table.add_row(row![
                format!(" {}", err.line()).to_string().blue().bold(),
                sepperator,
                text,
            ]);

            // Print error information
            table.add_row(row![
                "",
                sepperator,
                format!(
                    "{: >column$}^ {}",
                    "",
                    err,
                    column = err.column() - whitespaces
                )
                .red()
                .bold(),
            ]);
        }
    }

    table.printstd();

    Ok(())
}

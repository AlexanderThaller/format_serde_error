use anyhow::anyhow;
use colored::*;
use std::fmt;

#[cfg(test)]
mod test;

/// Amount of lines to show before and after the error line
const CONTEXT_LINES: usize = 3;

#[derive(Debug)]
pub struct SerdeError {
    input: String,
    message: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub enum ErrorTypes {
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
    pub fn new(input: String, err: impl Into<ErrorTypes>) -> Result<SerdeError, anyhow::Error> {
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
        let context_lines = CONTEXT_LINES;

        // Skip until we are amount of context lines before the error line (context)
        // plus the line with the error ( + 1)
        // Saturating sub if the error is in the first few line we can't take more
        // context
        let skip = usize::saturating_sub(self.line, context_lines + 1);

        // Take lines before and after (context * 2) plus the line with the error ( + 1)
        let take = context_lines * 2 + 1;

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

        let separator = " | ".blue().bold();

        // When we dont print the line_position we want to fill up the space not used by
        // the line_position with whitespaces instead
        let fill_line_position = format!("{: >fill$}", "", fill = self.line.to_string().len());

        // Want to avoid printing when we are not at the beginning of the line. For
        // example anyhow will write 'Error:' in front of the output before
        // printing the buffer
        writeln!(f)?;

        self.input
            .lines()
            .into_iter()
            .enumerate()
            .skip(skip)
            .take(take)
            .map(|(index, text)| {
                // Make the index start at 1 makes it nicer to work with
                // Also remove unnecessary whitespace in front of text
                (
                    index + 1,
                    text.chars().skip(whitespace_count).collect::<String>(),
                )
            })
            .try_for_each(|(line_position, text)| {
                self.format_line(
                    f,
                    line_position,
                    &text,
                    whitespace_count,
                    &separator,
                    &fill_line_position,
                )
            })?;

        Ok(())
    }

    fn format_line(
        &self,
        f: &mut fmt::Formatter<'_>,
        line_position: usize,
        text: &str,
        whitespace_count: usize,
        separator: &colored::ColoredString,
        fill_line_position: &str,
    ) -> Result<(), std::fmt::Error> {
        if line_position != self.line {
            self.format_context_line(f, text, separator, fill_line_position)
        } else {
            self.format_error_line(f, text, line_position, separator)?;
            self.format_error_information(f, whitespace_count, separator, fill_line_position)
        }
    }

    fn format_context_line(
        &self,
        f: &mut fmt::Formatter<'_>,
        text: &str,
        separator: &colored::ColoredString,
        fill_line_position: &str,
    ) -> Result<(), std::fmt::Error> {
        writeln!(f, " {}{}{}", fill_line_position, separator, text.yellow())
    }

    fn format_error_line(
        &self,
        f: &mut fmt::Formatter<'_>,
        text: &str,
        line_position: usize,
        separator: &colored::ColoredString,
    ) -> Result<(), std::fmt::Error> {
        writeln!(
            f,
            " {}{}{}",
            line_position.to_string().blue().bold(),
            separator,
            text
        )
    }

    fn format_error_information(
        &self,
        f: &mut fmt::Formatter<'_>,
        whitespace_count: usize,
        separator: &colored::ColoredString,
        fill_line_position: &str,
    ) -> Result<(), std::fmt::Error> {
        // Print whitespaces until we reach the column value of the message. We also
        // have to add the amount of whitespace infront of the other lines.
        let fill_column_position = format!(
            "{: >column$}^ {}",
            "",
            self.message,
            column = self.column - whitespace_count
        );

        writeln!(
            f,
            " {}{}{}",
            fill_line_position,
            separator,
            fill_column_position.red().bold(),
        )
    }
}

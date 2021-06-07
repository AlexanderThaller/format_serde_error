//! Serde error messages for humans.
//!
//! Format serde errors in a way to make it obvious where the error in the
//! source file was.
//!
//! Example:
//!
//! ```rust
//! use format_serde_error::SerdeError;
//!
//! #[derive(Debug, serde::Serialize, serde::Deserialize)]
//! struct Config {
//!     values: Vec<String>,
//! }
//! # fn main() {
//! #   #[cfg(feature = "serde_yaml")]
//! #   if let Err(err) = parse_config() {
//! #     eprintln!("{}", err)
//! #   }
//! # }
//!
//! # #[cfg(feature = "serde_yaml")]
//! fn parse_config() -> Result<Config, anyhow::Error> {
//!   let config_str = "values:
//!   - 'first'
//!   - 'second'
//!   - third:";
//!
//!   let config = serde_yaml::from_str::<Config>(config_str)
//!     .map_err(|err| SerdeError::new(config_str.to_string(), err))?;
//!
//!   Ok(config)
//! }
//! ```
//!
//! The output will be:
//! ```text
//! Error:
//!    | values:
//!    |   - 'first'
//!    |   - 'second'
//!  4 |   - third:
//!    |           ^ values[2]: invalid type: map, expected a string at line 4 column 10
//! ```
//! # Crate Features
//! ## `serde_yaml`
//! *Enabled by default:* yes
//!
//! Enables support for errors emitted by `serde_yaml`. Enables the
//! implementation to convert [`serde_yaml::Error`] to [`SerdeError`] using the
//! [`From`] trait. Also extends the [`ErrorTypes`] enum by
//! [`ErrorTypes::Yaml`].
//!
//! ## `serde_json`
//! *Enabled by default:* yes
//!
//! Enables support for errors emitted by `serde_json`. Enables the
//! implementation to convert [`serde_json::Error`] to [`SerdeError`] using the
//! [`From`] trait. Also extends the [`ErrorTypes`] enum by
//! [`ErrorTypes::Json`].
//!
//! ## `colored`
//! *Enabled by default:* yes
//!
//! Enables support for color output to a terminal using the [`colored`] crate.
//! Also enables the functions [`always_color`], [`never_color`],
//! [`set_coloring_mode`], [`use_environment`] and the enum [`ColoringMode`]
//! which allow changing the behavior of [`colored`].
//!
//! ## `graphemes_support`
//! *Enabled by default:* yes
//!
//! Enables proper support for grapheme cluster when contextualizing long error
//! lines. Without this feature the crate will just split the line using
//! [`std::str::Chars`]. This can mean that certain error messages won't get
//! formatted properly when a string contains unicode grapheme clusters. You can
//! check the test `test::context_long_line::graphemes_string` for an example.

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![warn(rust_2018_idioms, unused_lifetimes, missing_debug_implementations)]

#[cfg(feature = "colored")]
use colored::Colorize;

use std::fmt;

#[cfg(feature = "colored")]
mod control;

#[cfg(test)]
mod test;

#[cfg(feature = "colored")]
pub use control::{
    always_color,
    never_color,
    set_coloring_mode,
    use_environment,
    ColoringMode,
};

/// Amount of lines to show before and after the line containing the error.
pub const CONTEXT_LINES: usize = 3;

/// Amount of characters to show before and after the column containing the
/// error.
pub const CONTEXT_CHARACTERS: usize = 30;

/// Separator used between the line numbering and the lines.
const SEPARATOR: &str = " | ";

/// Ellipse used to indicated if a long line hase been contextualized.
const ELLIPSE: &str = "...";

/// Struct for formatting the error together with the source file to give a
/// nicer output.
#[derive(Debug)]
pub struct SerdeError {
    input: String,
    message: String,
    line: Option<usize>,
    column: Option<usize>,
}

/// Contains the error that will be used by [`SerdeError`] to format the output.
/// For this to work the error needs to support emitting the line and column of
/// the error. We are implementing [`Into`] for some common types. If a error
/// type is not implemented yet the [`ErrorTypes::Custom`] can be used instead.
#[derive(Debug)]
pub enum ErrorTypes {
    #[cfg(feature = "serde_json")]
    /// Contains [`serde_json::Error`].
    Json(serde_json::Error),

    #[cfg(feature = "serde_yaml")]
    /// Contains [`serde_yaml::Error`].
    Yaml(serde_yaml::Error),

    /// Used for custom errors that don't come from serde_yaml or
    /// serde_json.
    Custom {
        /// Error message that should be displayed.
        error: Box<dyn std::error::Error>,
        /// Line the error occured at.
        line: Option<usize>,
        /// Column the error occured at.
        column: Option<usize>,
    },
}

impl std::error::Error for SerdeError {}

impl fmt::Display for SerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for ErrorTypes {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

#[cfg(feature = "serde_yaml")]
impl From<serde_yaml::Error> for ErrorTypes {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Yaml(err)
    }
}

impl From<(Box<dyn std::error::Error>, Option<usize>, Option<usize>)> for ErrorTypes {
    fn from(value: (Box<dyn std::error::Error>, Option<usize>, Option<usize>)) -> Self {
        Self::Custom {
            error: value.0,
            line: value.1,
            column: value.2,
        }
    }
}

impl SerdeError {
    /// Create a new [`SerdeError`] from compatible serde errors. See
    /// [`ErrorTypes`] for more information.
    pub fn new(input: String, err: impl Into<ErrorTypes>) -> SerdeError {
        let error = err.into();

        let (message, line, column) = match error {
            #[cfg(feature = "serde_json")]
            ErrorTypes::Json(e) => (e.to_string(), Some(e.line()), Some(e.column())),

            #[cfg(feature = "serde_yaml")]
            ErrorTypes::Yaml(e) => match e.location() {
                // Don't set line/column if we don't have a location
                None => (e.to_string(), None, None),

                Some(location) => (
                    e.to_string(),
                    Some(location.line()),
                    Some(location.column()),
                ),
            },

            ErrorTypes::Custom {
                error,
                line,
                column,
            } => (error.to_string(), line, column),
        };

        Self {
            input,
            message,
            line,
            column,
        }
    }

    fn format(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // If line and column are not set we assume that we can't make a nice output
        // so we will just print the original message in red and bold
        if self.line.is_none() && self.column.is_none() {
            #[cfg(feature = "colored")]
            return writeln!(f, "{}", self.message.red().bold());

            #[cfg(not(feature = "colored"))]
            return writeln!(f, "{}", self.message);
        }

        let error_line = self.line.unwrap_or_default();
        let error_column = self.column.unwrap_or_default();

        // Amount of lines to show before and after the error line
        let context_lines = CONTEXT_LINES;

        // Skip until we are amount of context lines before the error line (context)
        // plus the line with the error ( + 1)
        // Saturating sub if the error is in the first few line we can't take more
        // context
        let skip = usize::saturating_sub(error_line, context_lines + 1);

        // Take lines before and after (context * 2) plus the line with the error ( + 1)
        let take = context_lines * 2 + 1;

        // Minimize the input to only what we need so we can reuse it without
        // having to iterate over the whole input again.
        let minimized_input = self.input.lines().skip(skip).take(take).collect::<Vec<_>>();

        // If the minimized_input is empty we can assume that the input was empty as
        // well. In that case we can't make a nice output so we will just print
        // the original message in red and bold
        if minimized_input.is_empty() {
            #[cfg(feature = "colored")]
            return writeln!(f, "{}", self.message.red().bold());

            #[cfg(not(feature = "colored"))]
            return writeln!(f, "{}", self.message);
        }

        // To reduce the amount of space text takes we want to remove unnecessary
        // whitespace in front of the text.
        // Find the line with the least amount of whitespace in front and use
        // that to remove the whitespace later.
        // We basically want to find the least indented line.
        // We cant just use trim as that would remove all whitespace and remove all
        // indentation.
        let whitespace_count = minimized_input
            .iter()
            .map(|line| line.chars().take_while(|s| s.is_whitespace()).count())
            .min()
            .unwrap_or_default();

        #[cfg(feature = "colored")]
        let separator = SEPARATOR.blue().bold();

        #[cfg(not(feature = "colored"))]
        let separator = SEPARATOR;

        // When we don't print the line_position we want to fill up the space not used
        // by the line_position with whitespace instead
        let fill_line_position = format!("{: >fill$}", "", fill = error_line.to_string().len());

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
                    error_line,
                    error_column,
                    &text,
                    whitespace_count,
                    &separator,
                    &fill_line_position,
                )
            })?;

        Ok(())
    }

    // TODO: Maybe make another internal struct for formatting instead of having
    // this list of args.
    #[allow(clippy::too_many_arguments)]
    fn format_line(
        &self,
        f: &mut fmt::Formatter<'_>,
        line_position: usize,
        error_line: usize,
        error_column: usize,
        text: &str,
        whitespace_count: usize,

        #[cfg(feature = "colored")] separator: &colored::ColoredString,

        #[cfg(not(feature = "colored"))] separator: &str,

        fill_line_position: &str,
    ) -> Result<(), std::fmt::Error> {
        if line_position == error_line {
            let (context_line, new_error_column, context_before, context_after) =
                Self::context_long_line(text, error_column, CONTEXT_CHARACTERS);

            Self::format_error_line(
                f,
                &context_line,
                line_position,
                separator,
                context_before,
                context_after,
            )?;

            self.format_error_information(
                f,
                whitespace_count,
                separator,
                fill_line_position,
                new_error_column,
                context_before,
            )
        } else {
            Self::format_context_line(f, text, separator, fill_line_position)
        }
    }

    fn format_error_line(
        f: &mut fmt::Formatter<'_>,
        text: &str,
        line_position: usize,
        #[cfg(feature = "colored")] separator: &colored::ColoredString,
        #[cfg(not(feature = "colored"))] separator: &str,
        context_before: bool,
        context_after: bool,
    ) -> Result<(), std::fmt::Error> {
        #[cfg(feature = "colored")]
        let line_pos = line_position.to_string().blue().bold();

        #[cfg(not(feature = "colored"))]
        let line_pos = line_position;

        write!(f, " {}{}", line_pos, separator)?;

        if context_before {
            #[cfg(feature = "colored")]
            write!(f, "{}", (ELLIPSE.blue().bold()))?;
            #[cfg(not(feature = "colored"))]
            write!(f, "{}", ELLIPSE)?;
        }

        write!(f, "{}", text)?;

        if context_after {
            #[cfg(feature = "colored")]
            write!(f, "{}", (ELLIPSE.blue().bold()))?;
            #[cfg(not(feature = "colored"))]
            write!(f, "{}", ELLIPSE)?;
        }

        writeln!(f)
    }

    fn format_error_information(
        &self,
        f: &mut fmt::Formatter<'_>,
        whitespace_count: usize,
        #[cfg(feature = "colored")] separator: &colored::ColoredString,

        #[cfg(not(feature = "colored"))] separator: &str,

        fill_line_position: &str,
        error_column: usize,
        context_before: bool,
    ) -> Result<(), std::fmt::Error> {
        let ellipse_space = if context_before { 3 } else { 0 };

        // Print whitespace until we reach the column value of the message. We also
        // have to add the amount of whitespace in front of the other lines.
        // If context_before is true we also need to add the space used by the ellipse
        let fill_column_position = format!(
            "{: >column$}^ {}",
            "",
            self.message,
            column = error_column - whitespace_count + ellipse_space
        );

        #[cfg(feature = "colored")]
        let fill_column_position = fill_column_position.red().bold();

        writeln!(
            f,
            " {}{}{}",
            fill_line_position, separator, fill_column_position,
        )
    }

    fn format_context_line(
        f: &mut fmt::Formatter<'_>,
        text: &str,
        #[cfg(feature = "colored")] separator: &colored::ColoredString,

        #[cfg(not(feature = "colored"))] separator: &str,

        fill_line_position: &str,
    ) -> Result<(), std::fmt::Error> {
        #[cfg(feature = "colored")]
        return writeln!(f, " {}{}{}", fill_line_position, separator, text.yellow());

        #[cfg(not(feature = "colored"))]
        return writeln!(f, " {}{}{}", fill_line_position, separator, text);
    }

    fn context_long_line(
        text: &str,
        error_column: usize,
        context_chars: usize,
    ) -> (String, usize, bool, bool) {
        #[cfg(feature = "graphemes_support")]
        use unicode_segmentation::UnicodeSegmentation;

        #[cfg(feature = "graphemes_support")]
        // As we could deal with unicode we can have characters that are multiple code
        // points. In that case we do not want to iterate over each code point
        // (i.e. using text.chars()) we need to use graphemes instead.
        let input = text.graphemes(true).collect::<Vec<_>>();

        #[cfg(not(feature = "graphemes_support"))]
        // If graphemes are not something we expect to deal with we can also just use chars
        // instead.
        let input = text.chars().collect::<Vec<_>>();

        // Skip until we are amount of context chars before the error column (context)
        // plus the column with the error ( + 1) Saturating sub if the error is
        // in the first few chars we can't take more context
        let skip = usize::saturating_sub(error_column, context_chars + 1);

        // Take chars before and after (context_chars * 2) plus the column with the
        // error ( + 1)
        let take = context_chars * 2 + 1;

        // If we skipped any characters that means we are contextualizing before the
        // error. That means that we need to print ... at the beginning of the error
        // line later on in the code.
        let context_before = skip != 0;

        // If the line is bigger than skipping and taking combined that means that we
        // not getting the remaining text of the line after the error. That
        // means that we need to print ... at the end of the error line later on
        // in the code.
        let context_after = skip + take < input.len();

        let minimized_input = input.into_iter().skip(skip).take(take).collect();

        // Error column has moved to the right as we skipped some characters so we need
        // to update it. Saturating sub as the error could be at the beginning
        // of the line.
        let new_error_column = usize::saturating_sub(error_column, skip);

        (
            minimized_input,
            new_error_column,
            context_before,
            context_after,
        )
    }
}

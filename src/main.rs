use colored::*;

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

    let lines = json_str.lines().collect::<Vec<_>>();
    let length = lines.len();
    let fill = number_length(length);

    let skip = usize::saturating_sub(err.line(), 4);
    let mut after = err.line() + 3;
    if after > length {
        after = length
    }

    for (index, line) in lines.into_iter().enumerate().skip(skip) {
        if index == after {
            break;
        }

        if index == err.line() - 1 {
            println!(
                " {}{: >fill$} {}{}",
                err.line().to_string().blue().bold(),
                "",
                "|".blue().bold(),
                line,
                fill = fill - number_length(index)
            );

            println!(
                " {: >fill$} {}{: >column$}{} {}",
                "",
                "|".blue().bold(),
                "",
                "^".red().bold(),
                err.to_string().red().bold(),
                fill = fill,
                column = err.column()
            );
        } else {
            println!(
                " {: >fill$} {}{}",
                "",
                "|".blue().bold(),
                line.yellow(),
                fill = fill
            );
        }
    }

    Ok(())
}

fn number_length(mut number: usize) -> usize {
    let mut len = 0;

    while number > 0 {
        number /= 10;
        len += 1;
    }

    len
}

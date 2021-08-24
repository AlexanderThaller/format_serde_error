#[derive(Debug)]
struct Config {
    values: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let _config_str = r#"values = [
	"first",
    "second",
    third=
]"#;

    todo!()
}

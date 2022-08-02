use std::error::Error;
use std::io::{stdin, stdout, Read, Write};
use toml::de::from_str;
use toml::ser::to_string_pretty;
use toml::value::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let value: Value = from_str(&input)?;

    let output = to_string_pretty(&value)?;
    stdout().write_fmt(format_args!("{output}"))?;

    Ok(())
}

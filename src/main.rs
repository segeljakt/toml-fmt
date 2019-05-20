use std::io::{self, Read, Write};
use std::error::Error;
fn main() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let value: toml::value::Value = toml::de::from_str(&input)?;
    let output = toml::ser::to_string_pretty(&value)?;
    io::stdout().write_fmt(format_args!("{}", output))?;
    Ok(())
}

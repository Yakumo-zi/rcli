use std::fmt::Display;
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Parser, Debug)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=verify_file)]
    pub input: String,
    #[arg(short, long, default_value = "output")]
    pub output: String,
    #[arg(long,value_parser=foramt_parse, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file(path: &str) -> Result<String> {
    if std::path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(anyhow::Error::msg(format!("File not found: {}", path)))
    }
}
fn foramt_parse(fromat_str: &str) -> Result<OutputFormat, anyhow::Error> {
    fromat_str.parse()
}
impl From<OutputFormat> for &str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

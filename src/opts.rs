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
    #[command(name = "genpass", about = "Generate password")]
    Genpass(GenpassOpts),

    #[command(subcommand)]
    Base64(Base64Cmd),
}

#[derive(Parser, Debug)]
pub enum Base64Cmd {
    #[command(name = "encode", about = "Encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode base64")]
    Decode(Base64DecodeOpts),
}
#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long,default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
impl From<Base64Format> for &str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=verify_file)]
    pub input: String,
    #[arg(short, long, default_value = "output")]
    pub output: String,
    #[arg(long, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Parser, Debug)]
pub struct GenpassOpts {
    #[arg(short, long)]
    pub length: usize,

    #[arg(long = "upper", default_value_t = true)]
    pub uppercase: bool,

    #[arg(long = "lower", default_value_t = true)]
    pub lowercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_file(path: &str) -> Result<String> {
    if path == "-" || std::path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(anyhow::Error::msg(format!("File not found: {}", path)))
    }
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

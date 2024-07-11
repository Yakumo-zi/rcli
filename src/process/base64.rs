use crate::opts::Base64Format;
use anyhow::{Ok, Result};
use base64::prelude::*;

fn get_reader(input: &str) -> Box<dyn std::io::Read> {
    if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input).unwrap())
    }
}

pub fn encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let res = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(buf),
    };
    println!("{}", res);
    Ok(())
}

pub fn decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let res = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buf)?,
    };
    let res = String::from_utf8(res)?;
    println!("{}", res);
    Ok(())
}

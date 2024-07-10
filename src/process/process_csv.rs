use anyhow::Result;
use csv::Reader;

use crate::opts::OutputFormat;
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value)
    }
    let file_name = format!("{}.{}", output, format);
    let mut writer = std::fs::File::create(file_name)?;
    match format {
        OutputFormat::Json => serde_json::to_writer_pretty(&mut writer, &ret)?,
        OutputFormat::Yaml => serde_yaml::to_writer(&mut writer, &ret)?,
    }
    Ok(())
}

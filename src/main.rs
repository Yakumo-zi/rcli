use anyhow::Result;
use clap::Parser;
use rcli::{decode, encode, Opts, SubCommand};
fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            rcli::process_csv(&csv_opts.input, csv_opts.output, csv_opts.format)?;
        }
        SubCommand::Genpass(genpass_opts) => rcli::gen_password(
            genpass_opts.length,
            genpass_opts.uppercase,
            genpass_opts.lowercase,
            genpass_opts.number,
            genpass_opts.symbol,
        )?,
        SubCommand::Base64(base64_cmd) => match base64_cmd {
            rcli::Base64Cmd::Encode(encode_opts) => encode(&encode_opts.input, encode_opts.format)?,
            rcli::Base64Cmd::Decode(decode_opts) => decode(&decode_opts.input, decode_opts.format)?,
        },
    }
    Ok(())
}

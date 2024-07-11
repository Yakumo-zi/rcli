mod opts;
mod process;

pub use opts::{
    Base64Cmd, Base64DecodeOpts, Base64EncodeOpts, CsvOpts, Opts, OutputFormat, SubCommand,
};
pub use process::{gen_password, process_csv, decode, encode};

use std::error::Error;

use data_validator_lib::run;
use data_validator_lib::{ArgsInner, FileFormatInner};
use derive_more::FromStr;
fn main() -> Result<(), Box<dyn Error>> {
    run()
}

#[derive(Debug, Clone, FromStr)]
pub enum FileFormat {
    JSON,
    JSONL,
    CSV,
    TSV,
}

impl From<FileFormat> for FileFormatInner {
    fn from(value: FileFormat) -> Self {
        match value {
            FileFormat::JSON => FileFormatInner::Json,
            FileFormat::JSONL => FileFormatInner::Jsonl,
            FileFormat::CSV => FileFormatInner::Csv,
            FileFormat::TSV => FileFormatInner::Tsv,
        }
    }
}

#[derive(clap::Parser, Debug)]
pub struct Args {
    /// Optional data format. If this is not specified, the file extension is used to deduce
    /// what the file format is.
    #[arg(long, short = 'F')]
    pub format: Option<FileFormat>,

    /// Data from stdin. Cannot be used with the `files` argument.
    #[arg(long, short = 'd', conflicts_with("files"))]
    pub data: Option<String>,

    /// Files and/or directories to validate.
    #[arg(long, short = 'f', conflicts_with("data"))]
    pub files: Option<Vec<String>>,
    /// Files and directories will be traversed recursively.
    #[arg(long, short = 'r', conflicts_with("data"))]
    pub recursive: bool,
}

impl From<Args> for ArgsInner {
    fn from(value: Args) -> Self {
        let format = value.format.map(FileFormatInner::from);
        Self {
            format,
            data: value.data,
            files: value.files,
            recursive: value.recursive,
        }
    }
}

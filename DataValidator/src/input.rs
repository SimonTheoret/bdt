use bytes::Bytes;
use futures::stream::BoxStream;
use std::{fmt::Debug, path::Path};

pub struct ByteStream<'a>(BoxStream<'a, Bytes>);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FileFormatInner {
    #[default]
    Json,
    Jsonl,
    Csv,
    Tsv,
    Parquet,
    Unknown,
}

pub trait Extension {
    fn file_ext(&self) -> FileFormatInner;
}

impl<S: AsRef<str>> Extension for S {
    fn file_ext(&self) -> FileFormatInner {
        let lower = self.as_ref().to_lowercase();
        if lower.ends_with(".json") {
            FileFormatInner::Json
        } else if lower.ends_with(".jsonl") {
            FileFormatInner::Jsonl
        } else if lower.ends_with(".csv") {
            FileFormatInner::Csv
        } else if lower.ends_with(".tsv") {
            FileFormatInner::Tsv
        } else if lower.ends_with(".parquet") {
            FileFormatInner::Parquet
        } else {
            FileFormatInner::Unknown
        }
    }
}
impl Extension for Path {
    fn file_ext(&self) -> FileFormatInner {
        match self.extension().map(|v| v.to_str()) {
            Some(Some("json")) => FileFormatInner::Json,
            Some(Some("jsonl")) => FileFormatInner::Jsonl,
            Some(Some("csv")) => FileFormatInner::Csv,
            Some(Some("tsv")) => FileFormatInner::Tsv,
            Some(Some("parquet")) => FileFormatInner::Parquet,
            _ => FileFormatInner::Unknown,
        }
    }
}

pub struct ArgsInner {
    pub format: Option<FileFormatInner>,
    pub data: Option<String>,
    pub files: Option<Vec<String>>,
    pub recursive: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;
    use pretty_assertions::assert_eq as passert_eq;

    #[rustfmt::skip]
        #[parameterized(
            string = {  Path::new("file.json"), Path::new("json.file"),   Path::new("file3.csv"), Path::new("thiscsv.tsv"), Path::new("faija"),       Path::new("jsoncsvtsv.txt")},
            expected = {FileFormatInner::Json,  FileFormatInner::Unknown, FileFormatInner::Csv,   FileFormatInner::Tsv,     FileFormatInner::Unknown, FileFormatInner::Unknown}
            )]
        fn find_extension_path(string: &Path, expected: FileFormatInner) {
            let format = string.file_ext();
            dbg!(string);
            passert_eq!(format, expected);
        }

    #[parameterized(
            string = {"file.json", "json.file", "file3.csv", "thiscsv.tsv", "faija", "jsoncsvtsv.txt"},
            expected = {FileFormatInner::Json, FileFormatInner::Unknown, FileFormatInner::Csv, FileFormatInner::Tsv, FileFormatInner::Unknown,FileFormatInner::Unknown}
            )]
    fn find_extension_str(string: &str, expected: FileFormatInner) {
        let format = string.file_ext();
        dbg!(string);
        passert_eq!(format, expected);
    }
}

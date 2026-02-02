use self::json::JsonValidator;
use crate::{ByteStream, FileFormatInner};

// mod csv;
mod json;
// mod jsonl;
// mod parquet;
// mod tsv;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Validator {
    Json(JsonValidator),
    // Jsonl(JsonlValidator),
    // Csv(CsvValidator),
    // Tsv(TsvValidator),
    // Parquet(ParquetValidtor),
}

impl Default for Validator {
    fn default() -> Self {
        Self::Json(JsonValidator::default())
    }
}

pub enum Validity {
    Valid,
    Invalid {
        data: Vec<u8>,
        line: Option<usize>,
        column: Option<usize>,
    },
}

impl From<FileFormatInner> for Validator {
    fn from(value: FileFormatInner) -> Self {
        match value {
            FileFormatInner::Json => Self::Json(JsonValidator::default()),
            _ => unimplemented!(),
            // FileFormatInner::Jsonl => Self::Jsonl(JsonlValidator::default()),
            // FileFormatInner::Csv => Self::Csv(CsvValidator::default()),
            // FileFormatInner::Tsv => Self::Tsv(TsvValidator::default()),
            // FileFormatInner::Parquet => Self::Parquet(ParquetValidator::default()),
            // FileFormatInner::Unknown => Self::Json(JsonValidator::default()),
        }
    }
}

impl Validator {
    fn validate_stream(&self, data: ByteStream) -> Validity {
        match self {
            Self::Json(v) => v.validate_stream(data),
            // Self::Jsonl(v) => v.validate_stream(data),
            // Self::Csv(v) => v.validate_stream(data),
            // Self::Tsv(v) => v.validate_stream(data),
            // Self::Parquet(v) => v.validate_stream(data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn unk_file_format_gets_converted_to_json_validator() {
        assert_eq!(
            Validator::from(FileFormatInner::Unknown),
            Validator::Json(JsonValidator::default())
        )
    }
}

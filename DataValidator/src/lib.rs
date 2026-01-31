use eyre::Result;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}

mod input {
    use derive_more::FromStr;
    use std::fmt::Debug;
    use std::io::Stdin;
    use std::path::{Path, PathBuf};

    trait DebugAsRefPath: Debug + AsRef<Path> {}

    #[derive(Debug, Clone, FromStr)]
    pub enum FileFormat {
        JSON,
        JSONL,
        CSV,
        TSV,
        Unknown,
    }
    #[derive(Debug)]
    pub enum DataSource {
        File(Box<dyn DebugAsRefPath>),
        Stdin(Stdin),
    }

    impl From<String> for DataSource {
        fn from(value: String) -> Self {
            todo!()
        }
    }

    #[derive(clap::Parser, Debug)]
    pub struct Args {
        format: Option<FileFormat>,
        data: String,
    }
}

pub enum DefaultValidator {
    Json,
}

impl DefaultValidator {
    pub fn json() -> DefaultValidator {
        Self::Json
    }
}

pub enum Res {
    Valid,
    Invalid { message: String },
}

pub trait Validator {
    fn validate<T>(&self, value: T) -> Result<Res>;
}

#[cfg(test)]
mod test {
    use super::*;

    fn json_corpus(validator: impl Validator) -> Res {
        let json = r#"[{"text"": "this is some text"}{"text"": "this is some other text"}]"#;
        validator.validate(json)
    }
}

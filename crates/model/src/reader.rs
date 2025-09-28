use std::io::Read;

use crate::{Error, ProjectDoc, error::DocError};

pub fn json_from_sb3_stream<R: Read>(
    handle: &mut R,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    loop {
        match zip::read::read_zipfile_from_stream(handle) {
            Ok(Some(file)) => {
                if file.name().to_lowercase().ends_with(".json") {
                    let value: serde_json::Value = serde_json::from_reader(file)?;
                    return Ok(value);
                }
            }
            Ok(None) => Err("no document")?,
            Err(e) => {
                log::error!("Error encountered while reading sb3: {e:?}");
                Err(DocError::Io(e.into()))?
            }
        }
    }
}

impl ProjectDoc {
    pub fn from_sb3_stream<R: Read>(handle: &mut R) -> Result<Self, DocError> {
        loop {
            match zip::read::read_zipfile_from_stream(handle) {
                Ok(Some(file)) => {
                    if file.name().to_lowercase().ends_with(".json") {
                        let value: serde_json::Value = serde_json::from_reader(file)?;
                        return Ok(Self::from_json(value)?);
                    }
                }
                Ok(None) => Err(DocError::NoDocument)?,
                Err(e) => {
                    log::error!("Error encountered while reading sb3: {e:?}");
                    Err(DocError::Io(e.into()))?
                }
            }
        }
    }
}

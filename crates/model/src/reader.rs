use std::io::Read;
use std::path::Path;

use crate::error::{DocError, Error, TargetError};
use crate::{ProjectDoc, Target};

#[allow(unused)]
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
    pub fn from_sb3_file(path: impl AsRef<Path>) -> Result<Self, DocError> {
        let path = path.as_ref();
        let mut handle =
            std::fs::File::open(path).map_err(|err| DocError::FileRead(path.to_path_buf(), err))?;
        loop {
            match zip::read::read_zipfile_from_stream(&mut handle) {
                Ok(Some(file)) => {
                    if file.name().to_lowercase().ends_with(".json") {
                        let value: serde_json::Value = serde_json::from_reader(file)?;
                        return Ok(Self::from_json(value)?);
                    }
                }
                Ok(None) => Err(DocError::NoDocument)?,
                Err(e) => {
                    log::error!("Error encountered while reading sb3 {path:?}: {e:?}");
                    Err(DocError::Io(e.into()))?
                }
            }
        }
    }

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
    pub fn from_json(doc: serde_json::Value) -> Result<ProjectDoc, Error> {
        use crate::ext::WithJsonContextExt;
        let semver = doc["meta"]["semver"].as_str().map(std::rc::Rc::from);
        let targets = doc["targets"]
            .as_array()
            .ok_or(TargetError::NoTargetsArray)
            .with_json(&doc)?;
        let targets: Result<Vec<Target>, _> = targets.iter().map(Target::from_json).collect();
        Ok(ProjectDoc {
            targets: targets?.into(),
            semver,
        })
    }
}

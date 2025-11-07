use std::io::Read;
use std::path::Path;

use crate::error::{DocError, Error, TargetError};
use crate::{ProjectDoc, Target};

#[allow(unused)]
pub fn json_from_sb3_stream<R: Read, T: std::fmt::Display + Sized>(
    handle: &mut R,
    tag: Option<T>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let tag = tag.map(|s| format!(" {s}")).unwrap_or_default();
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
                log::error!("Error encountered while reading sb3{tag}: {e:?}");
                Err(DocError::Io(e.into()))?
            }
        }
    }
}

impl ProjectDoc {
    /// Read a sb3 file by [`Path`]
    ///
    /// This is a convenience method that will:
    ///
    /// 1. Open the provided path
    ///     - `Err(`[`DocError::FileRead`]`)` in case of IO error
    /// 2. Extract `project.json`
    ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
    ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
    ///       `project.json` was found
    /// 3. Parse `project.json`
    ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
    /// 4. Read the information to generate a valid document
    ///     - `Err(`[`DocError::Model`]`)` if the read data is represents
    ///       a semantically invalid program
    ///
    pub fn from_sb3_file(path: impl AsRef<Path>) -> Result<Self, DocError> {
        // wrapper is generic and will be duplicated for every concrete type
        // while this implementation can be used for all of them
        fn from_sb3_file_impl(path: &Path) -> Result<ProjectDoc, DocError> {
            let mut handle = std::fs::File::open(path)
                .map_err(|err| DocError::FileRead(path.to_path_buf(), err))?;
            loop {
                match zip::read::read_zipfile_from_stream(&mut handle) {
                    Ok(Some(file)) => {
                        if file.name().to_lowercase().ends_with(".json") {
                            let value: serde_json::Value = serde_json::from_reader(file)?;
                            return Ok(ProjectDoc::from_json(value)?);
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
        let path = path.as_ref();
        from_sb3_file_impl(path)
    }
    /// Read sb3 data as a stream from an instance of [`Read`]
    /// with a suffix string used for context in error logs
    ///
    /// The suffix should be empty or start with a space
    /// (that's why this method is private and should be used only
    /// by functions of this type) and will be used to give additional
    /// context to the log messages for errors.
    /// This could for example be a file name.
    ///
    /// This is a convenience method that will:
    ///
    /// 1. Extract `project.json`
    ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
    ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
    ///       `project.json` was found
    /// 2. Parse `project.json`
    ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
    /// 3. Read the information to generate a valid document
    ///     - `Err(`[`DocError::Model`]`)` if the read data is represents
    ///       a semantically invalid program
    ///
    fn _inner_from_sb3_stream<R: Read>(
        handle: &mut R,
        suffix: &str,
    ) -> Result<ProjectDoc, DocError> {
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
                    log::error!("Error encountered while reading sb3{suffix}: {e:?}");
                    Err(DocError::Io(e.into()))?
                }
            }
        }
    }
    /// Read sb3 data as a stream from an instance of [`Read`]
    /// where a tag with extra context for log messages is provided
    ///
    /// This is a convenience method that will:
    ///
    /// 1. Extract `project.json`
    ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
    ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
    ///       `project.json` was found
    /// 2. Parse `project.json`
    ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
    /// 3. Read the information to generate a valid document
    ///     - `Err(`[`DocError::Model`]`)` if the read data is represents
    ///       a semantically invalid program
    ///
    pub fn from_tagged_sb3_stream<R: Read, T: std::fmt::Display>(
        handle: &mut R,
        tag: &T,
    ) -> Result<Self, DocError> {
        Self::_inner_from_sb3_stream(handle, &format!(" {tag}"))
    }

    /// Read sb3 data as a stream from an instance of [`Read`]
    ///
    /// (If you want to tag this stream with a filename
    /// use [`Self::from_tagged_sb3_stream`])
    ///
    /// This is a convenience method that will:
    ///
    /// 1. Extract `project.json`
    ///     - `Err(`[`DocError::Io`]`)` in case of IO error while reading the zip file
    ///     - `Err(`[`DocError::NoDocument`]`)` if file list ends before
    ///       `project.json` was found
    /// 2. Parse `project.json`
    ///     - `Err(`[`DocError::Json`]`)` if the `project.json` fails to parse
    /// 3. Read the information to generate a valid document
    ///     - `Err(`[`DocError::Model`]`)` if the read data is represents
    ///       a semantically invalid program
    ///
    pub fn from_sb3_stream<R: Read>(handle: &mut R) -> Result<Self, DocError> {
        Self::_inner_from_sb3_stream(handle, "")
    }
    /// Parses a document from a json value
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

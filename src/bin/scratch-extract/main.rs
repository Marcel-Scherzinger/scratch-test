mod cli;
use std::io::{Read, Write};

use crate::cli::Data;

fn extract_json_from_handle<R: Read, W: Write>(
    in_handle: &mut R,
    out_handle: &mut W,
) -> Result<bool, std::io::Error> {
    loop {
        match zip::read::read_zipfile_from_stream(in_handle) {
            Ok(Some(mut file)) => {
                log::debug!(
                    "found file {}: {} bytes ({} bytes packed)",
                    file.name(),
                    file.size(),
                    file.compressed_size()
                );
                if file.name().to_lowercase().ends_with(".json") {
                    std::io::copy(&mut file, out_handle)?;
                    return Ok(true);
                }
            }
            Ok(None) => return Ok(false),
            Err(e) => {
                log::error!("Error encountered while reading archive: {e:?}");
                return Err(std::io::Error::other(e));
            }
        }
    }
}

fn extract_model_from_handle<R: Read, W: Write>(
    in_handle: &mut R,
    out_handle: &mut W,
) -> Result<bool, Box<dyn std::error::Error>> {
    let model = model::ProjectDoc::from_sb3_stream(in_handle)?;
    out_handle.write_fmt(format_args!("{model:#?}"))?;
    Ok(true)
}

fn extract_from_handle<R: Read, W: Write>(
    in_handle: &mut R,
    out_handle: &mut W,
    mode: Option<Data>,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(match mode.unwrap_or(Data::Json) {
        Data::Json => extract_json_from_handle(in_handle, out_handle)?,
        Data::Model => extract_model_from_handle(in_handle, out_handle)?,
    })
}

fn main() {
    use clap::Parser;
    let args = cli::Cli::parse();
    let path = args.input.input_file;

    let _ = dotenvy::dotenv();
    env_logger::init();

    let result = if let Some(path) = path {
        let mut mode = args.select;
        let mut out_handle: Box<dyn Write> = match args.output_file {
            Some(out_path) => {
                let out_path = if let Some(out_path) = out_path {
                    out_path
                } else if let Some(Data::Model) = mode {
                    path.with_extension("model")
                } else {
                    path.with_extension("json")
                };
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    mode = Some(Data::Json);
                } else if path.extension().and_then(|s| s.to_str()) == Some("model") {
                    mode = Some(Data::Model);
                }
                Box::new({
                    if args.no_overwrite && out_path.exists() {
                        log::error!("file exists but -W is used (terminate): {out_path:?}");
                        std::process::exit(3);
                    } else if let Ok(f) = std::fs::File::create(&out_path) {
                        f
                    } else {
                        log::error!("file creation failed (terminate): {out_path:?}");
                        std::process::exit(4);
                    }
                })
            }
            None => Box::new(std::io::stdout().lock()),
        };
        let mut in_handle = match std::fs::File::open(&path) {
            Ok(i) => i,
            Err(err) => {
                log::error!("reading input file failed: {path:?}: {err}");
                std::process::exit(5);
            }
        };
        extract_from_handle(&mut in_handle, &mut out_handle, mode)
    } else {
        let stdin = std::io::stdin();
        let mut in_handle = stdin.lock();
        let mut out_handle: Box<dyn Write> = match args.output_file {
            Some(Some(out_path)) => Box::new(if args.no_overwrite && out_path.exists() {
                log::error!("file exists but -W is used (terminate): {out_path:?}");
                std::process::exit(3);
            } else if let Ok(f) = std::fs::File::create(&out_path) {
                f
            } else {
                log::error!("file creation failed (terminate): {out_path:?}");
                std::process::exit(4);
            }),
            Some(None) => {
                log::warn!("-o flag without path is noop when reading from stdin");
                Box::new(std::io::stdout().lock())
            }
            None => Box::new(std::io::stdout().lock()),
        };
        extract_from_handle(&mut in_handle, &mut out_handle, args.select)
    };
    if let Err(error) = result {
        log::error!("error occured: {error:#?}");
        std::process::exit(1);
    }
}

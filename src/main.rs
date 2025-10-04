mod cli;
mod visual;

use std::path::PathBuf;

use clap::Parser;
use testdata::{ExercisePart, ExerciseTest};
use visual::print_report;

use model::*;

use crate::cli::Cli;

fn run_single(file: PathBuf, exercise_number: u8, exercise_part: ExercisePart) {
    let tester: std::rc::Rc<dyn ExerciseTest> = match (exercise_number, exercise_part) {
        (1, ExercisePart::A) => std::rc::Rc::new(testdata::A1a),
        (1, ExercisePart::B) => std::rc::Rc::new(testdata::A1b),
        (2, ExercisePart::A) => std::rc::Rc::new(testdata::A2a),
        _ => todo!(),
    };

    if let Ok(mut content) = std::fs::File::open(&file) {
        let p = ProjectDoc::from_sb3_stream(&mut content);
        let file_name = file
            .file_name()
            .map(|os| os.to_string_lossy().clone())
            .unwrap_or_default();
        print_report(&file_name, tester.as_ref(), p, &file_name);
    } else {
        log::error!("Unable to open file {file:?}")
    }
}
fn run_submissions(path: PathBuf, exercise_number: u8) {
    let exercise_tests: Vec<Option<std::rc::Rc<dyn ExerciseTest>>> =
        match testdata::exercises(exercise_number) {
            Some(e) => e
                .iter()
                .enumerate()
                .inspect(|(idx, t)| {
                    if t.is_none() {
                        log::warn!(
                            "No test runner available for {}. part of exercise {exercise_number}",
                            idx + 1
                        );
                    }
                })
                .map(|(_, t)| t.clone())
                .collect(),
            None => {
                log::warn!("No tests found for exercise {exercise_number}");
                std::process::exit(1);
            }
        };

    let dir_entries = if let Ok(r) = std::fs::read_dir(&path) {
        r.flatten().filter(|e| e.path().is_dir())
    } else {
        log::error!("Unable to read directory: {path:?}");
        std::process::exit(1);
    };

    for folder in dir_entries {
        let folder_name = match folder.file_name().into_string() {
            Ok(s) => s,
            Err(err) => {
                log::error!("Folder has no valid utf-8 name: {err:?}");
                continue;
            }
        };
        let person_name = match folder_name.split_once("_") {
            Some((name, _)) => name,
            None => &folder_name,
        };

        let mut sb3_files: Vec<_> = match std::fs::read_dir(folder.path()) {
            Ok(r) => r
                .filter(|entry| {
                    entry
                        .as_ref()
                        .ok()
                        .and_then(|e| e.path().extension().map(|o| o.to_os_string()))
                        .and_then(|o| o.to_str().map(|s| s.to_string()))
                        == Some("sb3".to_string())
                })
                .flatten()
                .map(|e| e.path())
                .collect(),
            Err(_err) => {
                log::error!("Unable to read subdir: {:?}", folder.path());
                continue;
            }
        };
        alphanumeric_sort::sort_path_slice(&mut sb3_files);

        if exercise_tests.iter().len() != sb3_files.len() {
            let person_fmt = visual::pad_person(person_name);
            println!(
                "[MATC] {person_fmt} {} scratch files for {} exercises, match not possible",
                sb3_files.len(),
                exercise_tests.iter().len()
            );
            continue;
        }

        let parts = exercise_tests.iter().zip(&sb3_files);

        for (tester, path) in parts {
            if let Some(tester) = tester {
                if let Ok(mut content) = std::fs::File::open(path) {
                    let p = ProjectDoc::from_sb3_stream(&mut content);
                    print_report(
                        person_name,
                        tester.as_ref(),
                        p,
                        &path
                            .file_name()
                            .map(|s| s.to_string_lossy())
                            .unwrap_or_default(),
                    );
                } else {
                    log::error!("Unable to open file {path:?}")
                }
            } else {
                log::info!("No test runner for {exercise_number} available");
            }
        }
    }
}

fn main() {
    let _ = dotenvy::dotenv();
    env_logger::init();

    let args = Cli::parse();

    match args.commands {
        cli::Commands::Submissions { folder, exercise } => run_submissions(folder, exercise),
        cli::Commands::Single {
            file,
            exercise,
            part,
        } => {
            let part = if part.a || part.part == Some('a') {
                testdata::ExercisePart::A
            } else if part.b || part.part == Some('b') {
                testdata::ExercisePart::B
            } else if part.c || part.part == Some('c') {
                testdata::ExercisePart::C
            } else {
                std::process::exit(4);
            };
            run_single(file, exercise, part)
        }
    }
}

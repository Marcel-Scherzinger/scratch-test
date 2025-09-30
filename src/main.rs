mod cli;
mod visual;

use clap::Parser;
use testdata::ExerciseTest;
use visual::print_report;

use model::*;

use crate::cli::Cli;

fn main() {
    let _ = dotenvy::dotenv();
    env_logger::init();

    let args = Cli::parse();

    let path = args.folder;
    let exercise_number = args.exercise;

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
            let mut person_fmt = person_name.to_string();
            person_fmt.push(':');
            person_fmt.push_str(&" ".repeat(20 - person_fmt.chars().count()));
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

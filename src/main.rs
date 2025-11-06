//! # Scratch Interpreter
//!
//! This project provides a way to run algorithmic programs written using the
//! block-oriented programming language [Scratch](https://scratch.mit.edu/).
//!
//! The behaviour and output state of such programs can then be analysed
//! and unit-tested to report general implementation errors or give sometimes
//! even specific advice (this is up to the test definition code) on how to
//! fix errors.
//!
//! Everything is written in Rust and also the test definitions need to be in Rust.
//!
//! # Parts
//!
//! This projects is split into multiple crates to organise it's components:
//!
//! - [`model`] This is the bridge to the Scratch sb3 file format and parses it into
//!   an easy-to-use structure for further usage. Here, all blocks are defined and
//!   grouped by their function and kind. Some unsupported blocks are also classified
//!   as this property to be able to report this better.
//! - [`interpreter`] This is a virtual machine that uses the structure from [`model`]
//!   to run the program within set limits and constraints providing it with specified
//!   inputs like pre-generated random numbers or "answers" to questions which is a
//!   Scratch concept comparable to asking the user on stdin.
//! - [`testreports`] Here a general framework for reporting found errors and wrong
//!   behaviour is defined. You can define exercises that group tests in categories.
//!   Each entity (whole report or exercise/category/single test) is able to send
//!   arbitrary text as "messages" in different severity levels. Those messages will
//!   be shown to users depending on the pass/fail status of individual tests and
//!   categories. The usage of this framework is not required.
//!   [`model`] and [`interpreter`] can be used completly independently!
//! - [`testdata`] Here the default exercises are defined by using the framework from
//!   [`testreports`]
//! - [`scratch-yew`](../scratch_yew/index.html)
//!   This is a frontend-only application that compiles to WebAssembly (WASM)
//!   and is intended as an option for participants to test their solution
//!   before submitting it. It uses the data from [`testdata`] by the spec of [`testreports`]
//!   to visually group tests and especially failures by input types.

mod cli;
mod visual;

use colored::Colorize;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    rc::Rc,
};

use clap::Parser;
use testdata::{ExercisePart, ExerciseTest};
use visual::print_report;

use model::*;

use crate::cli::Cli;

fn run_id_intersection(path: PathBuf, ignore_siblings: bool, min_common: usize) {
    use walkdir::WalkDir;

    let mut tree: BTreeMap<(Rc<str>, Rc<str>), Vec<_>> = BTreeMap::new();

    let mut files = vec![];

    let m_path = path.clone();
    for (absentry, dispentry) in WalkDir::new(&m_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|o| o.to_os_string())
                .and_then(|o| o.to_str().map(|s| s.to_string()))
                == Some("sb3".to_string())
        })
        .flat_map(|entry| {
            let p = entry.path();
            p.strip_prefix(&m_path)
                .map(|o| o.to_path_buf())
                .map(|rel| (p.to_path_buf(), rel))
        })
    {
        let file_index = files.len();

        if let Ok(m) = model::ProjectDoc::from_sb3_file(&absentry) {
            files.push((absentry, dispentry, m.ids_with_blocks().count()));
            for (id, opcode) in m.ids_with_blocks() {
                tree.entry((id, opcode)).or_default().push(file_index);
            }
        }
    }

    let map: HashMap<_, _> = tree
        .into_iter()
        .filter(|(_key, v)| v.len() > 1)
        .flat_map(|(key, v)| {
            v.into_iter()
                .tuple_combinations()
                .map(move |(a, b)| ((a, b), key.clone()))
        })
        .into_group_map();

    for ((a, b), common) in map.into_iter().sorted() {
        if common.len() < min_common {
            continue;
        }

        let a_count = format!("{ :2}", files[a].2);
        let b_count = format!("{ :2}", files[b].2);
        let common_count = format!("{ :2}", common.len());
        let a_disp = files[a].1.to_string_lossy();
        let b_disp = files[b].1.to_string_lossy();

        let are_siblings = files[a].0.parent() == files[b].0.parent();

        if are_siblings && ignore_siblings {
            continue;
        }

        println!(
            "{} / {} \\ {} (A/shared\\B) {} {}",
            a_count.blue(),
            common_count.magenta(),
            b_count.yellow(),
            a_disp.blue(),
            b_disp.yellow()
        );
    }
}

fn run_single(file: PathBuf, exercise_number: u8, exercise_part: ExercisePart) {
    let tester: std::rc::Rc<dyn ExerciseTest> = match (exercise_number, exercise_part) {
        (1, ExercisePart::A) => std::rc::Rc::new(testdata::A1a),
        (1, ExercisePart::B) => std::rc::Rc::new(testdata::A1b),
        // (2, ExercisePart::A) => std::rc::Rc::new(testdata::A2a),
        _ => todo!(),
    };

    let p = ProjectDoc::from_sb3_file(&file);
    let file_name = file
        .file_name()
        .map(|os| os.to_string_lossy().clone())
        .unwrap_or_default();
    print_report(&file_name, tester.as_ref(), p, &file_name);
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
                let p = ProjectDoc::from_sb3_file(path);
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
        cli::Commands::FindIdIntersections {
            folder,
            no_ignore_siblings,
            min_common,
        } => run_id_intersection(folder, !no_ignore_siblings, min_common),
    }
}

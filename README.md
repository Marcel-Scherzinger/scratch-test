# Scratch tester
For running this application you need to [install Rust](https://rust-lang.org/tools/install/).

This takes a scratch program and provides it with "answers" as input while collecting it's output.
After completing the output can be compared to the one of a sample solution.

By now, no reading of test cases is implemented.

## Run
When you run `cargo run -- --folder FOLDER --exercise NUMBER`
the submissions in `FOLDER` will be tested as exercise `NUMBER`.

You can set the log level via the `RUST_LOG` environment variable (`.env` file).

## scratch-extract

In addition to the normal binary you can use:
```bash
cargo run --bin scratch-extract -- --help
```
to extract json data and the internal model from a `sb3` file.
(`--` ensures that cargo passes the following arguments to the binary itself.)


# Cargo Tokio

## Overview 
This cargo subcomand aims to simplify the development process. It gives an easy way
to perform locally the commands that are performed by the CI tool.

## Installation
```sh
cargo install cargo-tokio
```
## Examples
```sh
# runs **all** CISteps
cargo tokio
```
```sh
# runs loom tests
cargo tokio loom
```
```sh
# runs the test-unstable suite
cargo tokio test-unstable
```

## Project structure

`ci.rs`:
- `TokioCIStage` are commands typically defined through the `TokioCIStageBuilder`.
These are mostly `cargo test`, `cargo hack`, etc. Usually multiple `TokioCIStage`
make a `TokioCIStep`.
- `TokioCIStep` corresponds to a step in the CI. `miri`, `loom`, `san`, `clippy` would be
different CI steps.

`cli.rs`:  
- Command line options to run the tool.

`main.rs`:  
- Glues everything so that it can run as an executable.

## Contributing
Ideally this command should mirror any updates on the `.github/` directory on the
[tokio](https://github.com/tokio-rs/tokio) crate. So if there's anything missing, feel free
to submit a PR.


## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/cargo-tokio/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you, shall be licensed as MIT, without any additional
terms or conditions.

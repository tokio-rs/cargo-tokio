use cargo_tokio_ci::{TokioCIStageBuilder, TokioCIStep};
fn main() {
    TokioCIStep::test_tokio_full().unwrap();
    // let script = r#"
    // set -e
    // rustup override set nightly
    // rustup component add miri
    // cargo +nightly miri setup
    // "#
    // //&& rm -rf tokio/tests
    // .trim_start()
    // .trim_end();
}

use cargo_tokio_ci::cli::TokioCIStageBuilder;
fn main() {
    let mut tokio_stage = TokioCIStageBuilder::new("cargo")
        .test_features_full()
        .build();
    tokio_stage.run().unwrap();
}

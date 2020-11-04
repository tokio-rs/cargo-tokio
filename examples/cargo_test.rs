use cargo_tokio_ci::process::ProcessBuilder;

fn main() {
    let pb = ProcessBuilder::new("cargo", &["test", "--features", "full"]);
    let mut process = pb.build();
    process.run().unwrap();
}

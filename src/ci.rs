use std::{
    collections::VecDeque, convert::AsRef, ffi::OsStr, io::Result, iter::IntoIterator,
    process::Command,
};

use crate::targets::Target;

#[derive(PartialEq, Copy, Clone)]
enum ToolChain {
    Stable,
    Nightly,
}

pub struct TokioCIStep {
    toolchain: ToolChain,
    stages: VecDeque<TokioCIStage>,
}
pub struct TokioCIStage {
    cmd: Command,
}

pub struct TokioCIStageBuilder {
    cmd: Command,
}

impl TokioCIStageBuilder {
    pub(crate) fn new<S>(cmd: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        Self {
            cmd: Command::new(cmd),
        }
    }

    pub(crate) fn args<I>(self, args: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<OsStr>,
    {
        let mut new_self = self;
        new_self.cmd.args(args);
        new_self
    }

    pub(crate) fn envs<I, K, V>(self, envs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let mut new_self = self;
        new_self.cmd.envs(envs);
        new_self
    }

    pub(crate) fn current_dir<P>(self, cwd: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let mut new_self = self;
        new_self.cmd.current_dir(cwd);
        new_self
    }

    // start `test tokio full`
    pub(crate) fn test_features_full(self) -> Self {
        self.current_dir("tokio")
            .args(vec!["test", "--features", "full"])
    }

    pub(crate) fn test_all_features(self) -> Self {
        self.args(vec!["test", "--workspace", "--all-features"])
    }

    pub(crate) fn check_full_parking_lot(self) -> Self {
        self.current_dir("tokio")
            .args(vec!["test", "--features", "full,parking_lot"])
    }

    pub(crate) fn test_integration_each_feature(self) -> Self {
        self.current_dir("tests-integration")
            .args(vec!["hack", "test", "--each-feature"])
    }

    pub(crate) fn test_build_each_feature(self) -> Self {
        self.current_dir("tests-build")
            .args(vec!["hack", "test", "--each-feature"])
    }
    // end of `test tokio full`

    // start of `test tokio full --unstable`
    pub(crate) fn test_full_unstable(self) -> Self {
        self.current_dir("tokio")
            .args(vec!["test", "--features", "full"])
            .envs(vec![("RUSTFLAGS", "--cfg tokio_unstable -Dwarnings")])
    }

    // end of `test tokio full --unstable`

    // start miri
    pub(crate) fn miri(self) -> Self {
        self.current_dir("tokio").args(vec![
            "miri",
            "test",
            "--features",
            "rt,rt-multi-thread,sync",
            "task",
        ])
    }

    // start miri
    fn asan(self) -> Self {
        self.current_dir("tokio")
            //cargo test --all-features --target x86_64-unknown-linux-gnu --lib -- --test-threads 1
            .args(vec![
                "test",
                "--all-features",
                "--target",
                Target::x86_64_unknown_linux_gnu.into(),
                "--lib",
                "--",
                "--test-threads",
                "1",
            ])
            .envs(vec![
                ("RUSTFLAGS", "-Z sanitizer=address"),
                ("ASAN_OPTIONS", "detect_leaks=0"),
            ])
    }

    fn cross(self, target: Target) -> Self {
        self.args(vec!["check", "--workspace", "--target", target.into()])
    }

    pub(crate) fn features_check_each_feature(self) -> Self {
        self.args(vec![
            "hack",
            "check",
            "--all",
            "--each-feature",
            "-Z",
            "avoid-dev-deps",
        ])
        .envs(vec![("RUSTFLAGS", "--cfg tokio_unstable -Dwarnings")])
    }

    pub(crate) fn features_check_each_feature_unstable(self) -> Self {
        self.args(vec![
            "hack",
            "check",
            "--all",
            "--each-feature",
            "-Z",
            "avoid-dev-deps",
        ])
        .envs(vec![("RUSTFLAGS", "--cfg tokio_unstable -Dwarnings")])
    }

    pub(crate) fn minrust_test_workspace_all_features(self) -> Self {
        self.args(vec!["check", "--workspace", "--all-features"])
    }

    pub(crate) fn clippy(self) -> Self {
        self.args(vec!["clippy", "--all", "--tests"])
    }

    // docs
    pub(crate) fn docs(self) -> Self {
        self.args(vec!["doc", "--lib", "--no-deps", "--all-features"])
            .envs(vec![("RUSTFLAGS", "--cfg docsrs")])
    }

    pub(crate) fn loom(self, loom_scope: &str) -> Self {
        self.current_dir("tokio")
            .args(vec![
                "test",
                "--lib",
                "--release",
                "--features",
                "full",
                "--",
                "--nocapture",
                loom_scope,
            ])
            .envs(vec![(
                "RUSTFLAGS",
                "--cfg loom --cfg tokio_unstable -Dwarnings",
            )])
    }

    // loom missing
    pub(crate) fn build(self) -> TokioCIStage {
        let cmd = self.cmd;
        TokioCIStage { cmd }
    }
}
impl TokioCIStage {
    pub fn run(&mut self) -> Result<()> {
        let mut child = self.cmd.spawn()?;
        child.wait()?;
        Ok(())
    }
}

impl TokioCIStep {
    pub(crate) fn test_tokio_full() -> Result<()> {
        let mut q = VecDeque::new();
        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .test_features_full()
                .build(),
        );

        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .check_full_parking_lot()
                .build(),
        );

        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .test_all_features()
                .build(),
        );

        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .test_integration_each_feature()
                .build(),
        );

        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .test_build_each_feature()
                .build(),
        );

        Self::from((q, ToolChain::Stable)).run()
    }

    pub(crate) fn test_tokio_full_unstable() -> Result<()> {
        let mut q = VecDeque::new();
        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .test_full_unstable()
                .build(),
        );

        Self::from((q, ToolChain::Stable)).run()
    }

    pub(crate) fn miri() -> Result<()> {
        let mut q = VecDeque::new();
        q.push_back(TokioCIStageBuilder::new("cargo").miri().build());
        Self::from((q, ToolChain::Nightly)).run()
    }

    pub(crate) fn san() -> Result<()> {
        let mut q = VecDeque::new();
        q.push_back(TokioCIStageBuilder::new("cargo").asan().build());
        Self::from((q, ToolChain::Nightly)).run()
    }

    pub(crate) fn cross() -> Result<()> {
        let targets = [
            Target::i686_unknown_linux_gnu,
            Target::powerpc_unknown_linux_gnu,
            Target::powerpc64_unknown_linux_gnu,
            Target::mips_unknown_linux_gnu,
            Target::arm_linux_androideabi,
        ];
        let q = targets
            .iter()
            .map(|t| TokioCIStageBuilder::new("cargo").cross(t.clone()).build())
            .collect();
        Self::from((q, ToolChain::Stable)).run()
    }

    pub(crate) fn features() -> Result<()> {
        let script = "cargo install cargo-hack";
        std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .spawn()?
            .wait()?;

        let mut q = VecDeque::new();
        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .features_check_each_feature()
                .build(),
        );
        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .features_check_each_feature_unstable()
                .build(),
        );
        Self::from((q, ToolChain::Nightly)).run()
    }

    pub(crate) fn minrust() -> Result<()> {
        let mut q = VecDeque::new();
        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .minrust_test_workspace_all_features()
                .build(),
        );
        Self::from((q, ToolChain::Stable)).run() // TODO: validate since the yaml says env.minrust
    }

    pub(crate) fn fmt() -> Result<()> {
        // install clippy
        let script = "rustup component add rustfmt";
        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .spawn()?;

        child.wait()?;

        let script = r#"
        if ! rustfmt --check --edition 2018 $(find . -name '*.rs' -print); then
        printf "Please run \`rustfmt --edition 2018 \$(find . -name '*.rs' -print)\` to fix rustfmt errors.\nSee CONTRIBUTING.md for more details.\n" >&2
        exit 1
        fi        "#
        .trim_start()
        .trim_end();
        std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .spawn()?
            .wait()?;
        Ok(())
    }

    pub(crate) fn clippy() -> Result<()> {
        // install clippy
        let script = "rustup component add clippy";
        std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .spawn()?
            .wait()?;
        let mut q = VecDeque::new();
        q.push_back(TokioCIStageBuilder::new("cargo").clippy().build());
        Self::from((q, ToolChain::Stable)).run()
    }

    pub(crate) fn docs() -> Result<()> {
        let mut q = VecDeque::new();
        q.push_back(TokioCIStageBuilder::new("cargo").docs().build());
        Self::from((q, ToolChain::Nightly)).run()
    }

    pub(crate) fn loom() -> Result<()> {
        let q = [
            "--skip loom_pool", // TODO: not working!
            "loom_pool::group_a",
            "loom_pool::group_b",
            "loom_pool::group_c",
            "loom_pool::group_d",
        ]
        .iter()
        .map(|loom_scope| TokioCIStageBuilder::new("cargo").loom(loom_scope).build())
        .collect();
        Self::from((q, ToolChain::Stable)).run()
    }

    fn setup_toolchain(&mut self) -> Result<()> {
        let toolchain = self.toolchain;
        for stage in &mut self.stages {
            stage.cmd.envs(vec![("RUSTUP_TOOLCHAIN", String::from(toolchain))]);
        }
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        self.setup_toolchain()?;
        while let Some(mut stage) = self.stages.pop_front() {
            stage.run()?
        }
        Ok(())
    }
}

impl From<(VecDeque<TokioCIStage>, ToolChain)> for TokioCIStep {
    fn from(tpl: (VecDeque<TokioCIStage>, ToolChain)) -> Self {
        Self {
            stages: tpl.0,
            toolchain: tpl.1,
        }
    }
}

impl From<ToolChain> for String {
    fn from(tc: ToolChain) -> String {
        match tc {
            ToolChain::Stable => "stable".to_string(),
            ToolChain::Nightly => "nightly".to_string(),
        }
    }
}
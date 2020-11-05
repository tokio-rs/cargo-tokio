use std::{
    collections::VecDeque, convert::AsRef, ffi::OsStr, io::Result, iter::IntoIterator,
    process::Command,
};

pub struct TokioCIStep {
    stages: VecDeque<TokioCIStage>,
}
pub struct TokioCIStage {
    cmd: Command,
}

pub struct TokioCIStageBuilder {
    cmd: Command,
}

impl TokioCIStageBuilder {
    pub fn new<S>(cmd: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        Self {
            cmd: Command::new(cmd),
        }
    }

    pub fn args<I>(self, args: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<OsStr>,
    {
        let mut new_self = self;
        new_self.cmd.args(args);
        new_self
    }

    pub fn envs<I, K, V>(self, envs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let mut new_self = self;
        new_self.cmd.envs(envs);
        new_self
    }

    pub fn current_dir<P>(self, cwd: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let mut new_self = self;
        new_self.cmd.current_dir(cwd);
        new_self
    }

    // start `test tokio full`
    pub fn test_features_full(self) -> Self {
        self.current_dir("tokio")
            .args(vec!["test", "--features", "full"])
    }

    pub fn test_all_features(self) -> Self {
        self.args(vec!["test", "--workspace", "--all-features"])
    }

    pub fn check_full_parking_lot(self) -> Self {
        self.current_dir("tokio")
            .args(vec!["test", "--features", "full,parking_lot"])
    }

    pub fn test_integration_each_feature(self) -> Self {
        self.current_dir("tests-integration")
            .args(vec!["hack", "test", "--each-feature"])
    }

    pub fn test_build_each_feature(self) -> Self {
        self.current_dir("tests-build")
            .args(vec!["hack", "test", "--each-feature"])
    }
    // end of `test tokio full`

    // start of `test tokio full --unstable`
    pub fn test_full_unstable(self) -> Self {
        self.current_dir("tokio")
            .args(vec!["test", "--features", "full"])
            .envs(vec![("RUSTFLAGS", "--cfg tokio_unstable -Dwarnings")])
    }

    pub fn build(self) -> TokioCIStage {
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
    pub fn test_tokio_full() -> Result<()> {
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

        let mut step = Self { stages: q };
        step.run()
    }

    pub fn test_unstable() -> Result<()> {
        let mut q = VecDeque::new();
        q.push_back(
            TokioCIStageBuilder::new("cargo")
                .test_full_unstable()
                .build(),
        );

        let mut step = Self { stages: q };
        step.run()
    }

    fn run(&mut self) -> Result<()> {
        while let Some(mut stage) = self.stages.pop_front() {
            stage.run()?
        }
        Ok(())
    }
}

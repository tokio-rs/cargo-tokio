use std::{convert::AsRef, ffi::OsStr, io::Result, iter::IntoIterator, process::Command};

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

    pub fn test_all_features_full(self) -> Self {
        self.args(vec!["test", "--features", "full"])
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

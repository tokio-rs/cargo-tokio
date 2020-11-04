use std::{
    io::Result,
    process::{Child, Command, ExitStatus},
};

pub struct ProcessBuilder<'a> {
    exec_path: &'a str,
    args: &'a [&'a str],
}

impl<'a> ProcessBuilder<'a> {
    pub fn new(exec_path: &'a str, args: &'a [&'a str]) -> Self {
        Self { exec_path, args }
    }

    pub fn build(self) -> Process {
        let ProcessBuilder { exec_path, args } = self;

        let mut command = Command::new(exec_path);
        command.args(args);
        Process::new(command)
    }
}

pub struct Process {
    command: Command,
}

impl Process {
    fn new(command: Command) -> Self {
        Self { command }
    }

    fn spawn(&mut self) -> Result<Child> {
        self.command.spawn()
    }

    pub fn run(&mut self) -> Result<ExitStatus> {
        let mut child = self.spawn()?;
        child.wait()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mock_test_triggered_example() {
        assert_eq!(1, 1)
    }
}

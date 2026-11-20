use std::path::Path;
use std::process::Stdio;
use tokio::process::{Child, ChildStdin, Command};

struct EngineProcess {
    child: Child,
    stdin: ChildStdin,
}

impl EngineProcess {
    fn spawn(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdin = child.stdin.take().ok_or("engine stdin was not piped")?;

        Ok(Self { child, stdin })
    }
}
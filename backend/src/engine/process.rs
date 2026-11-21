use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

pub struct EngineProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

impl EngineProcess {
    pub fn spawn(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut child = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdin = child.stdin.take().ok_or("engine stdin was not piped")?;

        let stdout = child.stdout.take().ok_or("engine stdout was not piped")?;
        let stdout = BufReader::new(stdout).lines();

        Ok(Self { child, stdin, stdout })
    }

    pub async fn ping(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stdin.write_all(b"isready\n").await?;
        self.stdin.flush().await?;

        while let Some(line) = self.stdout.next_line().await? {
            if line.trim() == "readyok" {
                return Ok(());
            }
        }

        Err("engine closed stdout before readyok".into())
    }
}
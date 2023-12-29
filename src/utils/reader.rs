use tokio::io::{BufReader, AsyncBufReadExt};
use std::io::{Error, ErrorKind, BufRead};
use std::process::{Stdio, ChildStdout};

pub enum Status {
  Success,
  Failed
}

/// 外部プログラム構造体
pub struct Program {
  pub command: String,
  pub args: Vec<String>,
  pub dir: String
}

impl Program {
  /// 外部プロセスを実行する
  pub fn execute(&self) -> Result<(), std::io::Error> {
    let process = std::process::Command::new(&self.command)
    .current_dir(&self.dir)
    .args(&self.args)
    .stdout(Stdio::piped())
    .spawn()?;
    let stdout = process.stdout.ok_or_else(|| Error::new(ErrorKind::Other, ""))?;
    let mut buffer = String::from("");
    let reader: std::io::BufReader<ChildStdout> = std::io::BufReader::new(stdout);

    for line in reader.lines() {
      let l = line?;
      println!("[debug] {}", &l);
      buffer.push_str(&l);
    }

    Ok(())
  }

  /// プログラムを外部プロセスで実行する。
  /// 逐次標準出力の結果を得るが、最後に実行を待って終了ステータスを返す。
  #[tokio::main]
  pub async fn execute_async(&self) -> tokio::io::Result<std::process::ExitStatus> {
    let mut command = tokio::process::Command::new(&self.command)
      .args(&self.args)
      .current_dir(&self.dir)
      .stdout(Stdio::piped())
      .spawn()?;
    // FIXME: 適切なエラーハンドリングになっていないと思われる...
    let stdout = command.stdout.take().expect("");
    let reader = BufReader::new(stdout);
    let lines = reader.lines();
    tokio::pin!(lines);

    let mut result = String::from("");
    while let Some(line) = lines.next_line().await? {
      result.push_str(&line);
    }

    let status = command.wait().await?;

    Ok(status)
  }
}

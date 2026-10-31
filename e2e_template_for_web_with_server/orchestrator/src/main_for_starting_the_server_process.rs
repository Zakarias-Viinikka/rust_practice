//! Spawns the `server` binary on a given port and waits until it answers
//! on TCP. Returns a handle the caller can kill.

use std::path::PathBuf;
use std::time::Duration;

pub struct ServerProcess {
    pub child: tokio::process::Child,
}

pub async fn main_for_starting_the_server_process(
    port: u16,
) -> Result<ServerProcess, String> {
    let server_binary = sibling_binary_path("server")?;
    let child = tokio::process::Command::new(&server_binary)
        .arg("--port")
        .arg(port.to_string())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| format!("could not spawn {}: {e}", server_binary.display()))?;

    wait_until_port_answers(port, Duration::from_secs(10)).await?;
    Ok(ServerProcess { child })
}

fn sibling_binary_path(name: &str) -> Result<PathBuf, String> {
    let me = std::env::current_exe()
        .map_err(|e| format!("could not find current exe: {e}"))?;
    let dir = me
        .parent()
        .ok_or("current exe has no parent folder")?;
    Ok(dir.join(name))
}

async fn wait_until_port_answers(port: u16, timeout: Duration) -> Result<(), String> {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        if tokio::net::TcpStream::connect(("127.0.0.1", port))
            .await
            .is_ok()
        {
            return Ok(());
        }
        if std::time::Instant::now() >= deadline {
            return Err(format!(
                "server did not answer on 127.0.0.1:{port} within {timeout:?}"
            ));
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

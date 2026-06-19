use tokio::{
    process::Command,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    tokio::spawn(infinite_adb_check());
    sleep(Duration::from_secs(100)).await;
}

async fn infinite_adb_check() {
    loop {
        let (connected_to_adb, terminal_output) = check_if_connected_to_adb().await;
        if connected_to_adb {
            println!("We are connected {}", terminal_output);
        } else {
            println!("Not connected to any adb devices");
        }
        sleep(Duration::from_secs(5)).await;
    }
}

async fn check_if_connected_to_adb() -> (bool, String) {
    let cmd_result = Command::new("adb").arg("devices").output().await.unwrap();
    let cmd_result = String::from_utf8_lossy(&cmd_result.stdout);
    let terminal_output = cmd_result.clone().into_owned();
    let cmd_result = parse_terminal_output(cmd_result.into_owned());
    (cmd_result, terminal_output)
}

fn parse_terminal_output(cmd_result: String) -> bool {
    let where_text_pattern_starts =
        cmd_result.find("List of devices attached").unwrap() + "List of devices attached".len();
    let cmd_result = cmd_result[where_text_pattern_starts..].to_string();
    if !cmd_result.trim().is_empty() && !cmd_result.contains("offline") {
        return true;
    } else {
        return false;
    }
}

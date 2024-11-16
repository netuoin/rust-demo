use std::process::Command;

fn main() {
    Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .spawn()
        .expect("Failed to start v2ray");
}
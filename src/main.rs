use std::process::Command;

fn main() {
    // 通过v2ray的api获取到流量数据
    let output = Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .output()
        .expect("Failed to start v2ray");

    // 输出流量数据
    println!("{:?}", output);
}

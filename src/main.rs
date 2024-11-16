use std::process::Command;

fn main() {
    // 通过v2ray的api获取到流量数据
    let output = Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .output()
        .expect("Failed to start v2ray");

    // 解析v2ray的api返回的数据
    let output_str = String::from_utf8(output.stdout).unwrap();
    println!("{}", output_str);
}
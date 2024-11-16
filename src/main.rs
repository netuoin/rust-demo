use std::process::Command;

fn main() {
    // 通过v2ray的api获取到流量数据
    let output = Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .output()
        .expect("Failed to start v2ray");
    let output =  String::from_utf8_lossy(&output.stdout);
    // 打印第一行
    let output = output.lines().next().unwrap();
    println!("{}", output);
    // 打印第二行
    let output = output.lines().nth(1).unwrap();
    println!("{}", output);
}

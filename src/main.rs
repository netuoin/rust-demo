use std::process::Command;

fn main() {
    // 通过v2ray的api获取到流量数据
    let output = Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .output()
        .expect("Failed to start v2ray");

        //     Value       Name
        // 1   59.30KB     inbound>>>api>>>traffic>>>downlink
        // 2   32.92KB     inbound>>>api>>>traffic>>>uplink
        // 3   2.95MB      inbound>>>tcp>>>traffic>>>downlink
        // 4   2.39MB      inbound>>>tcp>>>traffic>>>uplink
        // 5   2.44MB      outbound>>>direct>>>traffic>>>downlink
        // 6   2.26MB      outbound>>>direct>>>traffic>>>uplink
        // 7   2.44MB      user>>>userA>>>traffic>>>downlink
        // 8   2.26MB      user>>>userA>>>traffic>>>uplink
        

    // 输出流量数据,转换为json
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.replace(" ", "");
    let output = output.replace(">>>", "_");
    let output = output.replace("Value", "\"Value\"");
    let output = output.replace("Name", "\"Name\"");
    let output = output.replace("\n", ",");
    let output = format!("{{{}}}", output);
    println!("{}", output);
}

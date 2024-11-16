use std::process::Command;

fn main() {
    // 通过v2ray的api获取到流量数据
    let output = Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .output()
        .expect("Failed to start v2ray");

    //     Value       Name
    //     59.30KB     a1
    //     32.92KB     a2
    //     2.95MB      a3
    //     2.39MB      a4
    //     2.44MB      a5
    //     2.26MB      a6
    //     2.44MB      a7
    //     2.26MB      a8

    // 输出流量数据,转换为json格式
    // {"name":"a1","value":"59.30KB"}
    // {"name":"a2","value":"32.92KB"}
    // {"name":"a3","value":"2.95MB"}
    // {"name":"a4","value":"2.39MB"}
    // {"name":"a5","value":"2.44MB"}
    // {"name":"a6","value":"2.26MB"}
    // {"name":"a7","value":"2.44MB"}
    // {"name":"a8","value":"2.26MB"}
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

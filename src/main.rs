use std::process::Command;

fn main() {
    // 通过v2ray的api获取到流量数据
    let output = Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .output()
        .expect("Failed to start v2ray");

    // 控制台输入
    // Value       Name
    // 59.30KB     a1
    // 32.92KB     a2
    // 2.95MB      a3
    // 2.39MB      a4
    // 2.44MB      a5
    // 2.26MB      a6
    // 2.44MB      a7
    // 2.26MB      a8

    // 转换为json
    // {
    //     "a1": "59.30KB",
    //     "a2": "32.92KB",
    //     "a3": "2.95MB",
    //     "a4": "2.39MB",
    //     "a5": "2.44MB",
    //     "a6": "2.26MB",
    //     "a7": "2.44MB",
    //     "a8": "2.26MB"
    // }
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.split("\n").collect::<Vec<&str>>();
    let mut json = "{".to_string();
    for i in 1..output.len() {
        let line = output[i].split_whitespace().collect::<Vec<&str>>();
        if line.len() == 2 {
            json.push_str(&format!("\"{}\": \"{}\",\n", line[1], line[0]));
        }
    }
    json.pop();
    json.pop();
    json.push('}');
    println!("{}", json);
}

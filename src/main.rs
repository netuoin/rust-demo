use std::process::Command;

fn main() {
    // 通过v2ray的api获取到流量数据
    let output = Command::new("v2ray")
        .arg("api")
        .arg("stats")
        .output()
        .expect("Failed to start v2ray");
    // 接收命令行参数
    let args: Vec<String> = std::env::args().collect();
    // 赋值给变量
    let mut traffic: f64 = 0.0;
    // 判断是否有参数
    if args.len() > 1 {
        // 判断参数是否为数字
        if let Ok(num) = args[1].parse::<f64>() {
            // 赋值给变量
            traffic = num;
        }
    }
    // 打印
    println!("traffic: {}", traffic);
}

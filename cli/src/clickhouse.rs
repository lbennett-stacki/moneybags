use std::process::{Command, Stdio};

pub const CLICKHOUSE_DB_NAME: &str = "moneybags";

pub fn start() {
    println!("Starting clickhouse server");
    Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg("clickhouse-server")
        .arg("-p")
        .arg("8123:8123")
        .arg("-p")
        .arg("9000:9000")
        .arg("clickhouse/clickhouse-server")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn stop() {
    println!("Stopping clickhouse server");
    Command::new("docker")
        .arg("stop")
        .arg("clickhouse-server")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn remove() {
    println!("Removing clickhouse server");
    Command::new("docker")
        .arg("rm")
        .arg("clickhouse-server")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn client() {
    println!("Opening clickhouse client");
    Command::new("clickhouse")
        .arg("client")
        .arg("--host")
        .arg("localhost")
        .arg("--port")
        .arg("9000")
        .arg("--database")
        .arg(CLICKHOUSE_DB_NAME)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn query(query: String) {
    println!("Querying clickhouse server");
    Command::new("clickhouse")
        .arg("client")
        .arg("--host")
        .arg("localhost")
        .arg("--port")
        .arg("9000")
        .arg("--database")
        .arg(CLICKHOUSE_DB_NAME)
        .arg("--query")
        .arg(query)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

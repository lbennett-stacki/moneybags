use std::process::{Command, Stdio};

pub const CLICKHOUSE_DB_NAME: &str = "moneybags";

fn start_server() {
    Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--network")
        .arg("dragonfly-network")
        .arg("--name")
        .arg("clickhouse-server")
        .arg("-p")
        .arg("8123:8123")
        .arg("-p")
        .arg("9000:9000")
        .arg("-v")
        .arg("clickhouse-data:/var/lib/clickhouse")
        .arg("clickhouse/clickhouse-server")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn stop_server() {
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

fn start_ui() {
    Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg("ch-ui")
        .arg("-p")
        .arg("5521:5521")
        .arg("-e")
        .arg("VITE_CLICKHOUSE_URL=http://localhost:8123")
        .arg("-e")
        .arg("VITE_CLICKHOUSE_USER=default")
        .arg("-e")
        .arg("VITE_CLICKHOUSE_PASSWORD=")
        .arg("ghcr.io/caioricciuti/ch-ui:latest")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn stop_ui() {
    Command::new("docker")
        .arg("stop")
        .arg("ch-ui")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn remove_server() {
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

fn remove_ui() {
    Command::new("docker")
        .arg("rm")
        .arg("ch-ui")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn start() {
    println!("Starting clickhouse server");
    start_server();
    start_ui();
    println!("Clickhouse will be available at http://localhost:5521");
    println!("Clickhouse UI will be available at http://localhost:8123");
}

pub fn stop() {
    println!("Stopping clickhouse server");
    stop_server();
    stop_ui();
}

pub fn remove() {
    println!("Removing clickhouse server");
    remove_server();
    remove_ui();
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

pub fn start_safe() {
    stop();
    remove();
    start();
}

pub fn remove_safe() {
    stop();
    remove();
}

pub fn open_ui() {
    Command::new("open")
        .arg("http://localhost:5521")
        .spawn()
        .unwrap();
}

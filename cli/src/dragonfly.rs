use std::process::{Command, Stdio};

fn create_network() {
    Command::new("docker")
        .args(["network", "create", "dragonfly-network"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn start_server() {
    create_network();

    Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--network")
        .arg("dragonfly-network")
        .arg("--name")
        .arg("dragonfly-server")
        .arg("-p")
        .arg("6379:6379")
        .arg("-v")
        .arg("dragonfly-data:/data")
        .arg("docker.dragonflydb.io/dragonflydb/dragonfly")
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
        .arg("--network")
        .arg("dragonfly-network")
        .arg("--name")
        .arg("redisinsight")
        .arg("-p")
        .arg("5540:5540")
        .arg("-v")
        .arg("redisinsight-data:/data")
        .arg("redislabs/redisinsight:latest")
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
        .arg("dragonfly-server")
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
        .arg("redisinsight")
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
        .arg("dragonfly-server")
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
        .arg("redisinsight")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn remove_network() {
    Command::new("docker")
        .args(["network", "rm", "dragonfly-network"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn start() {
    println!("Starting DragonFlyDB server and RedisInsight");
    start_server();
    start_ui();
    println!("DragonFlyDB server will be available at http://localhost:6379");
    println!("RedisInsight will be available at http://localhost:5540");
}

pub fn stop() {
    println!("Stopping DragonFlyDB server and RedisInsight");
    stop_server();
    stop_ui();
}

pub fn remove() {
    println!("Removing DragonFlyDB server and RedisInsight");
    remove_server();
    remove_ui();
    remove_network();
}

pub fn client() {
    println!("Opening dragonfly-cli");
    Command::new("dragonfly-cli")
        .arg("-h")
        .arg("localhost")
        .arg("-p")
        .arg("6379")
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
        .arg("http://localhost:5540")
        .spawn()
        .unwrap();
}

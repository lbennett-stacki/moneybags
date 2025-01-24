use crate::db::client::db_client;
use crate::token::queries::has_token;
use crate::utils::blocking::blocking_call;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn test_specific_token_and_tx() {
    let token = "49Gy6L2cz616ZqEt3c4eMEjgbVpdstherwZJfaShpump";
    let tx =
        "5Q6Hn1osCBCWGuNxgHFmBS5NvN6gHJRrXZeDGapdD4VNb82kL6K1tB2VXQBrT92pVn92VZs4nY4vN7CRQ87yckTb";

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "moneybags-extractors-price",
            "--",
            "--token",
            token,
            "--tx",
            tx,
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    thread::sleep(Duration::from_secs(2));

    let db = db_client();

    let token = blocking_call(async { has_token(&db, token).await.unwrap() });

    assert!(token);

    println!("test tokens res {:?}", token);
}

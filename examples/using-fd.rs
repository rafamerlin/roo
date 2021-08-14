use std::io::{self, Write};
use std::process::Command;

fn main() -> () {
    println!("Hi from Simple");

    let output = Command::new("fd")
        .arg("-e")
        .arg("sln")
        .arg(".")
        .arg("../../pb")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();

    // io::stderr().write_all(&output.stderr).unwrap();
    let resp = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = resp.lines().into_iter().map(|x| x).collect();

    println!("This is the content inside resp: {:?}", lines[0]);

    assert!(output.status.success());
}

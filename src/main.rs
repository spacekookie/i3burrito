use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

const DEFAULT_FORTUNE_LENGTH: usize = 160;

fn fortune(len: Option<usize>) -> Vec<String> {
    let len_string = len
        .unwrap_or(DEFAULT_FORTUNE_LENGTH)
        .to_string();
    String::from_utf8_lossy(
        &Command::new("fortune")
            .args(&["-s", "-n", &len_string])
            .output()
            .expect("failed to execute `fortune`")
            .stdout,
    ).split('\n')
    .filter(|l| l != &"")
    .map(|s| s.replace("'", "\'"))
    .map(|s| s.into())
    .collect()
}

fn main() {
    let text = fortune(Some(6));
    let bash = include_str!("template.sh");

    let mut s = String::new();
    let mut y = 150;
    for line in text {
        s.push_str("-annotate ");
        s.push_str(&format!("+0+{} ", y));
        s.push_str(&format!("\"{}\" ", line));
        y += 50;
    }

    match fs::remove_file("/tmp/locker.sh") {
        Ok(_) => {}
        Err(_) => {}
    }

    let new = bash.replace("REPLACE_ME_HERE", &s);
    let mut f = File::create("/tmp/locker.sh").unwrap();
    f.write_all(new.as_bytes()).unwrap();

    Command::new("bash")
        .arg("/tmp/locker.sh")
        .output()
        .expect("Failed to run bash command!");
}

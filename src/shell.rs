use std::fs;
use std::process::Command;

pub fn exec(cmd: &str) -> String {
    let _sh = Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute process");

    match _sh.status.success() {
        true => return String::from_utf8_lossy(&_sh.stdout).to_string(),
        false => return String::from_utf8_lossy(&_sh.stderr).to_string(),
    }
}

pub fn clean(x: String) -> String {
    x.replace("\n", "")
}

pub fn slurp(f: &str) -> String {
    let mut content = String::new();
    if let Ok(file) = fs::read_to_string(&f.to_string()) {
        if let Ok(data) = file.parse() {
            content = data;
        }
    }
    content
}

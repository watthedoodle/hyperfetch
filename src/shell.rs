use std::process::Command;

pub fn exec(cmd: &str) -> String {
    let _sh = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute process");

    match _sh.status.success() {
        true => return String::from_utf8_lossy(&_sh.stdout).to_string(),
        false => return String::from_utf8_lossy(&_sh.stderr).to_string(),
    }
}

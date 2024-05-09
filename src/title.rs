use crate::shell;

pub fn get() -> String {
    let user = shell::clean(shell::exec("id -un"));
    let hostname = shell::clean(shell::exec("hostname"));
    format!("{}@{}", user, hostname)
}
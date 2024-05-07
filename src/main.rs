use std::io;
use std::io::Write;
mod kernel;
mod logos;

// original bash neofetch source: https://github.com/dylanaraps/neofetch/blob/master/neofetch
fn main() {
    let os = kernel::uname();
    println!(
        "------------------------------------------------------------------------------------"
    );
    println!("{}", os.sysname);
    println!("{}", os.release);
    println!("{}", os.machine);
    println!("{}", os.nodename);
    println!("{}", os.version);
    println!(
        "------------------------------------------------------------------------------------"
    );

    let mut logo = String::new();

    if os.version.to_lowercase().contains("ubuntu") {
        logo = logos::ubuntu();
    }
    if os.version.to_lowercase().contains("arch") {
        logo = logos::ubuntu();
    }
    if os.version.to_lowercase().contains("garuda") {
        logo = logos::garuda();
    }

    let _ = io::stdout().write_all(&format!("{}", logo).as_bytes());
    println!("");
}

fn reset() -> &'static str {
    "\x1B[0m"
}

use std::io;
use std::io::Write;
mod distros;
mod kernel;
mod logos;
mod os;

// original bash neofetch source: https://github.com/dylanaraps/neofetch/blob/master/neofetch
fn main() {
    println!(
        "------------------------------------------------------------------------------------"
    );

    let machine = os::detect();
    println!("OS detect output -> {:?}", machine);

    println!(
        "------------------------------------------------------------------------------------"
    );

    // let mut logo = String::new();

    // if os.version.to_lowercase().contains("ubuntu") {
    //     logo = logos::ubuntu();
    // }
    // if os.version.to_lowercase().contains("arch") {
    //     logo = logos::ubuntu();
    // }
    // if os.version.to_lowercase().contains("garuda") {
    //     logo = logos::garuda();
    // }
    let _ = io::stdout().write_all(&format!("{}", logos::arch()).as_bytes());

    println!("");
}

fn reset() -> &'static str {
    "\x1B[0m"
}

use std::io;
use std::io::Write;
mod distros;

// original bash neofetch source: https://github.com/dylanaraps/neofetch/blob/master/neofetch

fn main() {
    let _ = io::stdout().write_all(&format!("{}", distros::arch()).as_bytes());
}

fn reset() -> &'static str {
    "\x1B[0m"
}
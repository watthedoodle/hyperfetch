use clap::Parser;
use std::io;
use std::io::Write;

mod distros;
mod kernel;
mod logos;
mod os;
mod shell;
mod title;
mod uptime;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Wat The Dooodle",
    version,
    about = "A re-imagined neofetch in Rust"
)]
struct Arguments {
    /// Which distro's ascii art to display
    #[clap(long("ascii_distro"))]
    ascii_distro: Option<String>,
}

// original bash neofetch source: https://github.com/dylanaraps/neofetch/blob/master/neofetch
// ansci cursor codes https://notes.burke.libbey.me/ansi-escape-codes/
fn main() {
    let args = Arguments::parse();

    let machine = os::detect();
    let kernel = kernel::uname();
    let distro = distros::get(os::detect());

    let logo = match args.ascii_distro {
        Some(x) => logos::detect(&x),
        None => logos::detect(&distro),
    };

    let w = logo.width;
    let h = logo.height;
    let ascii = logo.ascii;

    self::render(ascii);
    self::render(self::move_cursor_up(h));
    self::newline_with_width(w);
    self::render(self::bold_on());
    self::render(title::get());
    self::newline_with_width(w);
    self::render(self::reset());
    self::render(format!("-----------------"));
    self::newline_with_width(w);
    self::render(self::bold_on());
    self::render(self::info("OS", &format!("{} {}", distro, kernel.machine)));
    self::newline_with_width(w);
    self::render(self::info("Host", "???"));
    self::newline_with_width(w);
    self::render(self::info("Kernel", &kernel.release));
    self::newline_with_width(w);
    self::render(self::info("Uptime", &uptime::get(machine)));
    self::newline_with_width(w);
    self::render(self::info("Packages", "???"));
    self::newline_with_width(w);
    self::render(self::info("Shell", "???"));
    self::newline_with_width(w);
    self::render(self::info("Resolution", "???"));
    self::newline_with_width(w);
    self::render(self::info("DE", "???"));
    self::newline_with_width(w);
    self::render(self::info("WM", "???"));
    self::newline_with_width(w);
    self::render(self::info("WM Theme", "???"));
    self::newline_with_width(w);
    self::render(self::info("Theme", "???"));
    self::newline_with_width(w);
    self::render(self::info("Icons", "???"));
    self::newline_with_width(w);
    self::render(self::info("Terminal", "???"));
    self::newline_with_width(w);
    self::render(self::info("CPU", "???"));
    self::newline_with_width(w);
    self::render(self::info("GPU", "???"));
    self::newline_with_width(w);
    self::render(self::info("Memory", "???"));

    if h > 18 {
        let excess = h - 18;
        for _num in 0..excess {
            println!("");
        }
        self::print_end();
    } else {
        println!("");
        self::print_end();
    }
}

fn red() -> String {
    format!("\x1B[31m")
}

fn render(x: String) {
    let _ = io::stdout().write_all(&format!("{}", x).as_bytes());
}

fn print_end() {
    println!("");
    println!("");
}

fn newline_with_width(x: u16) {
    self::render(self::move_cursor_down(1));
    self::render(self::move_cursor_back(9999));
    self::render(self::move_cursor_forward(x));
}

fn move_cursor_up(x: u16) -> String {
    format!("\x1B[{}A", x)
}

fn move_cursor_down(x: u16) -> String {
    format!("\x1B[{}B", x)
}

fn move_cursor_forward(x: u16) -> String {
    format!("\x1B[{}C", x)
}

fn move_cursor_back(x: u16) -> String {
    format!("\x1B[{}D", x)
}

fn move_cursor_to(x: u16, y: u16) -> String {
    format!("\x1B[{};{}H", x, y)
}

fn bold_on() -> String {
    format!("\x1B[1m")
}

fn reset() -> String {
    format!("\x1B[0m")
}

fn info(k: &str, v: &str) -> String {
    format!(
        "{}{}{}{}:{} {}",
        self::reset(),
        self::bold_on(),
        self::red(),
        k,
        self::reset(),
        v
    )
}

use clap::Parser;
use std::io;
use std::io::Write;

mod ansi;
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

    ansi::render(ascii);
    ansi::render(ansi::move_cursor_up(h));
    ansi::newline_with_width(w);
    ansi::render(ansi::bold_on());
    ansi::render(title::get());
    ansi::newline_with_width(w);
    ansi::render(ansi::reset());
    ansi::render(format!("-----------------"));
    ansi::newline_with_width(w);
    ansi::render(ansi::bold_on());
    ansi::render(ansi::info("OS", &format!("{} {}", distro, kernel.machine)));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Host", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Kernel", &kernel.release));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Uptime", &uptime::get(machine)));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Packages", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Shell", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Resolution", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("DE", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("WM", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("WM Theme", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Theme", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Icons", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Terminal", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("CPU", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("GPU", "???"));
    ansi::newline_with_width(w);
    ansi::render(ansi::info("Memory", "???"));

    if h > 18 {
        let excess = h - 18;
        for _num in 0..excess {
            println!("");
        }
        ansi::print_end();
    } else {
        println!("");
        ansi::print_end();
    }
}

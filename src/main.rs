use clap::Parser;


mod distros;
mod kernel;
mod logos;
mod os;
mod shell;
mod title;
mod uptime;
mod stdout_utils;

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

    stdout_utils::render(ascii);
    stdout_utils::render(stdout_utils::move_cursor_up(h));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::bold_on());
    stdout_utils::render(title::get());
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::reset());
    stdout_utils::render(format!("-----------------"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::bold_on());
    stdout_utils::render(stdout_utils::info("OS", &format!("{} {}", distro, kernel.machine)));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Host", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Kernel", &kernel.release));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Uptime", &uptime::get(machine)));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Packages", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Shell", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Resolution", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("DE", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("WM", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("WM Theme", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Theme", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Icons", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Terminal", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("CPU", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("GPU", "???"));
    stdout_utils::newline_with_width(w);
    stdout_utils::render(stdout_utils::info("Memory", "???"));

    if h > 18 {
        let excess = h - 18;
        for _num in 0..excess {
            println!("");
        }
        stdout_utils::print_end();
    } else {
        println!("");
        stdout_utils::print_end();
    }
}
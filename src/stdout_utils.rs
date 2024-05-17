use std::io;
use std::io::Write;


pub fn red() -> String {
    format!("\x1B[31m")
}

pub fn render(x: String) {
    let _ = io::stdout().write_all(&format!("{}", x).as_bytes());
}

pub fn print_end() {
    println!("");
    println!("");
}

pub fn newline_with_width(x: u16) {
    self::render(self::move_cursor_down(1));
    self::render(self::move_cursor_back(9999));
    self::render(self::move_cursor_forward(x));
}

pub fn move_cursor_up(x: u16) -> String {
    format!("\x1B[{}A", x)
}

pub fn move_cursor_down(x: u16) -> String {
    format!("\x1B[{}B", x)
}

pub fn move_cursor_forward(x: u16) -> String {
    format!("\x1B[{}C", x)
}

pub fn move_cursor_back(x: u16) -> String {
    format!("\x1B[{}D", x)
}

pub fn move_cursor_to(x: u16, y: u16) -> String {
    format!("\x1B[{};{}H", x, y)
}

pub fn bold_on() -> String {
    format!("\x1B[1m")
}

pub fn reset() -> String {
    format!("\x1B[0m")
}

pub fn info(k: &str, v: &str) -> String {
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

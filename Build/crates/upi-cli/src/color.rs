use std::sync::OnceLock;

pub fn green(s: &str) -> String {
    colorize(s, Color::Green)
}

pub fn bright_green(s: &str) -> String {
    colorize(s, Color::BrightGreen)
}

pub fn dim(s: &str) -> String {
    colorize(s, Color::Dim)
}

pub fn bold(s: &str) -> String {
    colorize(s, Color::Bold)
}

pub fn red(s: &str) -> String {
    colorize(s, Color::Red)
}

fn colorize(s: &str, color: Color) -> String {
    if !supports_color() {
        return s.to_string();
    }
    match color {
        Color::Green => format!("\x1b[38;2;5;150;105m{s}\x1b[0m"),
        Color::BrightGreen => format!("\x1b[38;2;52;211;153m{s}\x1b[0m"),
        Color::Dim => format!("\x1b[2m{s}\x1b[0m"),
        Color::Bold => format!("\x1b[1m{s}\x1b[0m"),
        Color::Red => format!("\x1b[38;2;239;68;68m{s}\x1b[0m"),
    }
}

enum Color {
    Green,
    BrightGreen,
    Dim,
    Bold,
    Red,
}

fn supports_color() -> bool {
    static SUPPORTED: OnceLock<bool> = OnceLock::new();
    *SUPPORTED.get_or_init(|| {
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }
        if let Ok(val) = std::env::var("COLORTERM") {
            if val == "truecolor" || val == "24bit" {
                enable_vt();
                return true;
            }
        }
        if std::env::var("WT_SESSION").is_ok() {
            enable_vt();
            return true;
        }
        #[cfg(not(windows))]
        {
            if let Ok(val) = std::env::var("TERM") {
                if val != "dumb" {
                    return true;
                }
            }
        }
        false
    })
}

/// Enable Windows VT processing so ANSI escape sequences render as colors.
#[cfg(windows)]
fn enable_vt() {
    use std::os::windows::io::AsRawHandle;

    unsafe {
        for raw in [
            std::io::stdout().as_raw_handle(),
            std::io::stderr().as_raw_handle(),
        ] {
            let handle = raw as Handle;
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                SetConsoleMode(handle, mode | 0x0004);
            }
        }
    }
}

#[cfg(not(windows))]
fn enable_vt() {}

#[cfg(windows)]
type Handle = *mut core::ffi::c_void;

#[cfg(windows)]
extern "system" {
    fn GetConsoleMode(h: Handle, mode: *mut u32) -> i32;
    fn SetConsoleMode(h: Handle, mode: u32) -> i32;
}

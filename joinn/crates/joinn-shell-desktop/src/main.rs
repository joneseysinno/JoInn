//! `joinn-desktop`: the desktop shell.

#![forbid(unsafe_code)]

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("joinn-desktop: usage: joinn-desktop <path to a .body>");
        std::process::exit(1);
    };
    if let Err(e) = joinn_shell_desktop::run(&path) {
        eprintln!("joinn-desktop: {e}");
        std::process::exit(1);
    }
}

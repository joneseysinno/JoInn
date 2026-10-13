//! `joinn-desktop`: the desktop shell.

#![forbid(unsafe_code)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dna = args
        .iter()
        .position(|a| a == "--strand" || a == "--fold")
        .map(|i| (args[i] == "--fold", args.get(i + 1)));
    if let Some((folded, path)) = dna {
        let Some(path) = path else {
            eprintln!(
                "joinn-desktop: usage: joinn-desktop --strand|--fold <path to a .strand or .chromosome>"
            );
            std::process::exit(1);
        };
        if let Err(e) = joinn_shell_desktop::run_dna(path, folded) {
            eprintln!("joinn-desktop: {e}");
            std::process::exit(1);
        }
        return;
    }
    let (lens, rest): (Option<&str>, Vec<&String>) = match args.iter().position(|a| a == "--lens") {
        Some(i) => (
            args.get(i + 1).map(String::as_str),
            args.iter()
                .enumerate()
                .filter(|(j, _)| *j != i && *j != i + 1)
                .map(|(_, a)| a)
                .collect(),
        ),
        None => (None, args.iter().collect()),
    };
    let Some(path) = rest.first() else {
        eprintln!(
            "joinn-desktop: usage: joinn-desktop <path to a .body, .contact, .system, .universe, .strand or .chromosome> [--lens NAME] | --strand <file> | --fold <file>"
        );
        std::process::exit(1);
    };
    if let Err(e) = joinn_shell_desktop::run(path, lens) {
        eprintln!("joinn-desktop: {e}");
        std::process::exit(1);
    }
}

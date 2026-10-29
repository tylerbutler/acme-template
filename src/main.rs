use std::process::ExitCode;

const HELP: &str = concat!(
    "Usage: ",
    env!("CARGO_PKG_NAME"),
    " [OPTIONS]\n\nOptions:\n  -h, --help     Show help\n  -V, --version  Show version\n"
);

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [] => print!("{HELP}"),
        [arg] if arg == "--help" || arg == "-h" => print!("{HELP}"),
        [arg] if arg == "--version" || arg == "-V" => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("error: unsupported arguments; use --help");
            return ExitCode::from(2);
        }
    }
    ExitCode::SUCCESS
}

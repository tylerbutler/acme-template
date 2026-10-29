use std::process::Command;

#[test]
fn shows_help_without_arguments_or_with_help_flag() {
    for args in [vec![], vec!["--help"], vec!["-h"]] {
        let output = Command::new(env!("CARGO_BIN_EXE_acme-template"))
            .args(args)
            .output()
            .unwrap();

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Usage: acme-template"));
        assert!(stdout.contains("--help"));
        assert!(stdout.contains("--version"));
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn shows_version_with_either_version_flag() {
    for arg in ["--version", "-V"] {
        let output = Command::new(env!("CARGO_BIN_EXE_acme-template"))
            .arg(arg)
            .output()
            .unwrap();

        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            concat!("acme-template ", env!("CARGO_PKG_VERSION"), "\n")
        );
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn rejects_unknown_or_extra_arguments() {
    for args in [
        vec!["--unknown"],
        vec!["run"],
        vec![""],
        vec!["--help", "--version"],
        vec!["--version", "extra"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_acme-template"))
            .args(args)
            .output()
            .unwrap();

        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("error:"));
        assert!(stderr.contains("--help"));
    }
}

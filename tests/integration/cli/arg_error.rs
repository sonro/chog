use super::util::{assert_stderr, test_program};

const SEE_HELP_MSG: &str = "See `--help` option for usage information.";

fn check_stderr(expected: &str, args: &[&str]) {
    let output = test_program(args, "");
    assert_stderr(
        &format!("Argument error: {}\n\n{}\n", expected, SEE_HELP_MSG),
        &output,
    );
}

#[test]
fn invalid_versions() {
    let invalids = [
        "maj",
        "min",
        "pat",
        "!",
        "1.c",
        "1",
        "1.1.1badformat",
        "1.2.a",
        "balh",
    ];
    for v in invalids {
        check_stderr(&format!("invalid custom version: `{}`", v), &[v]);
    }
}

#[test]
fn no_versions() {
    let invalid_args = [
        vec!["--force", "--dry-run"],
        vec!["-df"],
        vec!["-q"],
        vec!["-p", "path"],
        vec!["--output", "output"],
    ];
    for args in invalid_args {
        check_stderr("no version provided", &args);
    }
}

#[test]
fn unknown_flags() {
    let invalid_flags = ["-u", "-yi", "-yh", "-a", "--invalid", "--invalid"];
    for flag in invalid_flags {
        check_stderr(&format!("unknown flag: `{}`", flag), &[flag]);
    }
}

#[test]
fn no_paths() {
    let invalid_args = [
        vec!["--path"],
        vec!["patch", "--path"],
        vec!["--output"],
        vec!["-po"],
    ];
    for args in invalid_args {
        check_stderr("expected path for given options", &args);
    }
}

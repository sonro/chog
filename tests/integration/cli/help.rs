use super::util::{assert_stderr, assert_stdout, test_program};

#[test]
fn short_help_flag_prints_usage_to_stdout() {
    let output = test_program(&["-h"], "");
    assert_stderr("", &output);
    assert_stdout(EXPECTED_HELP, &output);
}

#[test]
fn long_help_flag_prints_usage_to_stdout() {
    let output = test_program(&["--help"], "");
    assert_stderr("", &output);
    assert_stdout(EXPECTED_HELP, &output);
}

#[test]
fn short_help_flag_separated_from_other_args() {
    let output = test_program(&["-q", "-h"], "");
    assert_stderr("", &output);
    assert_stdout(EXPECTED_HELP, &output);
}

#[test]
fn short_help_flag_mixed_with_other_flags() {
    let output = test_program(&["-qhf"], "");
    assert_stderr("", &output);
    assert_stdout(EXPECTED_HELP, &output);
}

#[test]
fn long_help_flag_separated_from_other_args() {
    let output = test_program(&["--quiet", "--help"], "");
    assert_stderr("", &output);
    assert_stdout(EXPECTED_HELP, &output);
}

const EXPECTED_HELP: &str = r#"chog 0.1.0

USAGE:
    chog [OPTIONS] [VERSION]

VERSIONS:
    major 
        Increase the major number - x.*.*

    minor
        Increase the minor number - *.x.*

    patch
        Increase the patch number - *.*.x

    [0-9].[0-9].[0-9]*
        Specify your own next version. For example: chog 0.2.3

OPTIONS:
    -h, --help
        Print this help output.

    -i, --info
        Print information about latest changelog version.

    -q, --quiet
        Minimal output.

    -f, --force
        No user confirmation.

    -p, --path <path>
        Specifiy the file path to the existing changelog.
        Default: ./CHANGELOG.md
    
    -o --output <path>
        Specifiy the file path of the changed file. 
        Default: ./CHANGELOG.md or --path value
        Using this option disables user confirmation.
     
    -d --dry-run
        Direct changed file to STDOUT.
        Using this option disables user confirmation.

"#;

use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

pub fn test_program(args: &[&str], input: &str) -> Output {
    let mut full_args = vec!["run", "-q"];
    let target = std::env::var("RUSTC_TARGET");
    if let Ok(ref target) = target {
        full_args.extend_from_slice(&["--target", target]);
    }
    full_args.push("--");
    full_args.extend_from_slice(args);

    // setup program args, stdout and stderr
    let mut cmd = Command::new("cargo");
    cmd.args(full_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // setup stdin
    cmd.stdin(match input.is_empty() {
        true => Stdio::null(),
        false => Stdio::piped(),
    });

    // start program
    let mut child = cmd
        .spawn()
        .unwrap_or_else(|err| panic!("Failed to execute program for testing: {}", err));

    if !input.is_empty() {
        // pipe in input
        let mut stdin = child.stdin.take().expect("Failed to get stdin handle");
        let input = input.as_bytes().to_owned();
        std::thread::spawn(move || {
            stdin.write_all(&input).expect("Failed to write to stdin");
        });
    }

    // run program to completion and return output
    child
        .wait_with_output()
        .unwrap_or_else(|err| panic!("Failed to wait for program to finish: {}", err))
}

pub fn assert_stdout(expected: &str, output: &Output) {
    let actual = String::from_utf8_lossy(&output.stdout);
    assert_output(expected, &actual, output);
}

pub fn assert_stderr(expected: &str, output: &Output) {
    let actual = String::from_utf8_lossy(&output.stderr);
    assert_output(expected, &actual, output);
}

fn assert_output(expected: &str, actual: &str, output: &Output) {
    assert_eq!(
        expected, actual,
        "\n# EXPECTED:\n{}\n# ACTUAL:\n{}\n{:?}",
        expected, actual, output
    );
}

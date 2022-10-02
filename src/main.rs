use cli::App;

mod cli;

const USAGE: &str = r#"chog 0.1.0

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = cli::App::new(&args);
    match res {
        Ok(app) if app.help => print!("{}", USAGE),
        Err(err) => eprint!("Argument error: {}\n{}", err, USAGE),
        Ok(app) => run(&app),
    }
}

fn run(_app: &App) {}

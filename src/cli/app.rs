use std::path::Path;

use chog::Version;

use super::Error;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct App<'a> {
    pub help: bool,
    pub info: bool,
    pub quiet: bool,
    pub force: bool,
    pub dry_run: bool,
    pub version: Version<'a>,
    pub in_file: Option<&'a Path>,
    pub out_file: Option<&'a Path>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            help: false,
            info: false,
            quiet: false,
            force: false,
            dry_run: false,
            version: Version::Patch,
            in_file: None,
            out_file: None,
        }
    }
}

impl<'a> App<'a> {
    pub fn new<S: AsRef<str>>(args: &'a [S]) -> Result<Self, Error> {
        let mut app = Self::default();
        if args.is_empty() {
            app.info = true;
            return Ok(app);
        }
        let mut args = args.iter().map(|a| a.as_ref()).peekable();

        let mut version = None;

        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                if app.handle_long_flag(arg, args.peek())? {
                    // consume next arg
                    args.next();
                }
            } else if arg.starts_with('-') {
                if app.handle_short_flag(arg, args.peek())? {
                    // consume next arg
                    args.next();
                }
            } else {
                version = Some(match arg {
                    "major" => Version::Major,
                    "minor" => Version::Minor,
                    "patch" => Version::Patch,
                    arg => custom_version_from_arg(arg)?,
                });
            }
        }

        if app.info || app.help {
            Ok(app)
        } else if version.is_none() {
            Err(Error::NoVersion)
        } else {
            app.version = version.unwrap();
            Ok(app)
        }
    }

    fn handle_long_flag(&mut self, arg: &'a str, next: Option<&&'a str>) -> Result<bool, Error> {
        let mut used_next = false;
        match arg {
            "--help" => self.help = true,
            "--info" => self.info = true,
            "--quiet" => self.quiet = true,
            "--force" => self.force = true,
            "--dry-run" => self.dry_run = true,
            "--path" => {
                self.in_file = Some(path_from_arg(next)?);
                used_next = true;
            }
            "--output" => {
                self.out_file = Some(path_from_arg(next)?);
                used_next = true
            }
            _ => return Err(Error::UnknownFlag(arg.into())),
        }
        Ok(used_next)
    }

    fn handle_short_flag(&mut self, arg: &str, next: Option<&&'a str>) -> Result<bool, Error> {
        let mut used_next = false;
        for ch in arg.chars().skip(1) {
            match ch {
                'h' => self.help = true,
                'i' => self.info = true,
                'q' => self.quiet = true,
                'f' => self.force = true,
                'd' => self.dry_run = true,
                'p' => {
                    self.in_file = Some(path_from_arg(next)?);
                    used_next = true;
                }
                'o' => {
                    self.out_file = Some(path_from_arg(next)?);
                    used_next = true
                }
                _ => return Err(Error::UnknownFlag(arg.into())),
            }
        }
        Ok(used_next)
    }
}

fn path_from_arg<'a>(next_arg: Option<&&'a str>) -> Result<&'a Path, Error> {
    match next_arg {
        Some(&arg) => Ok(Path::new(arg)),
        None => Err(Error::NoPath),
    }
}

fn custom_version_from_arg(arg: &str) -> Result<Version, Error> {
    let mut i = 0;
    for part in arg.split('.') {
        match i {
            0 | 1 => {
                if !part.chars().all(char::is_numeric) {
                    // major or minor part does not contain number
                    return Err(Error::InvalidVersion(arg.into()));
                }
            }
            2 => {
                if !part.chars().all(char::is_numeric) {
                    if let Some(patch) = part.split('-').next() {
                        if !patch.chars().all(char::is_numeric) {
                            return Err(Error::InvalidVersion(arg.into()));
                        }
                    } else {
                        // invalid patch format
                        return Err(Error::InvalidVersion(arg.into()));
                    }
                }
            }
            _ => return Err(Error::InvalidVersion(arg.into())),
        }
        i += 1;
    }
    if i != 3 {
        Err(Error::InvalidVersion(arg.into()))
    } else {
        Ok(Version::Custom(arg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(expected: Expected, args: &[&str]) {
        let actual = App::new(args).expect("test should not error");
        let expected = expected.build();
        assert_eq!(
            expected, actual,
            "\nEXEPECTED {:#?}\nACTUAL {:#?}\n",
            expected, actual
        );
    }

    fn check_error(expected: Error, args: &[&str]) {
        let actual = App::new(args).expect_err("test should error");
        assert_eq!(
            expected, actual,
            "\nEXEPECTED {:#?}\nACTUAL {:#?}\n",
            expected, actual
        );
    }

    #[test]
    fn default() {
        let actual = App::default();
        let expected = App {
            help: false,
            info: false,
            quiet: false,
            force: false,
            dry_run: false,
            version: Version::Patch,
            in_file: None,
            out_file: None,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_args_should_set_info() {
        check(Expected::new().info(), &[]);
    }

    #[test]
    fn patch_version() {
        check(Expected::new().patch(), &["patch"]);
    }

    #[test]
    fn minor_version() {
        check(Expected::new().minor(), &["minor"]);
    }

    #[test]
    fn major_version() {
        check(Expected::new().major(), &["major"]);
    }

    #[test]
    fn short_info_flag() {
        check(Expected::new().info(), &["-i"]);
    }

    #[test]
    fn long_info_flag() {
        check(Expected::new().info(), &["--info"]);
    }

    #[test]
    fn short_help_flag() {
        check(Expected::new().help(), &["-h"]);
    }

    #[test]
    fn long_help_flag() {
        check(Expected::new().help(), &["--help"]);
    }

    #[test]
    fn short_quiet_flag() {
        check(Expected::new().quiet(), &["-q", "patch"]);
    }

    #[test]
    fn long_quiet_flag() {
        check(Expected::new().quiet(), &["--quiet", "patch"]);
    }

    #[test]
    fn short_force_flag() {
        check(Expected::new().force(), &["-f", "patch"]);
    }

    #[test]
    fn long_force_flag() {
        check(Expected::new().force(), &["--force", "patch"]);
    }

    #[test]
    fn short_dry_run_flag() {
        check(Expected::new().dry_run(), &["-d", "patch"]);
    }

    #[test]
    fn long_dry_run_flag() {
        check(Expected::new().dry_run(), &["--dry-run", "patch"]);
    }

    #[test]
    fn short_in_file() {
        check(Expected::new().in_file("path"), &["-p", "path", "patch"]);
    }

    #[test]
    fn long_in_file() {
        check(
            Expected::new().in_file("path"),
            &["--path", "path", "patch"],
        );
    }

    #[test]
    fn short_out_file() {
        check(Expected::new().out_file("path"), &["-o", "path", "patch"]);
    }

    #[test]
    fn long_out_file() {
        check(
            Expected::new().out_file("path"),
            &["--output", "path", "patch"],
        );
    }

    #[test]
    fn short_in_file_and_out_file_same_path() {
        let file = "changelog.md";
        check(
            Expected::new().in_file(file).out_file(file),
            &["-po", file, "patch"],
        );
    }

    #[test]
    fn mixed_short_flags() {
        check(
            Expected::new().force().quiet().dry_run(),
            &["-fqd", "patch"],
        );
    }

    #[test]
    fn mixed_short_flags_with_help() {
        check(Expected::new().force().quiet().dry_run().help(), &["-fqdh"]);
    }

    #[test]
    fn mixed_short_flags_with_info() {
        check(Expected::new().force().quiet().dry_run().info(), &["-fqid"]);
    }

    #[test]
    fn short_out_file_error_no_path() {
        check_error(Error::NoVersion, &["-o", "patch"]);
    }

    #[test]
    fn long_out_file_error_no_path() {
        check_error(Error::NoPath, &["patch", "--output"]);
    }

    #[test]
    fn short_in_file_error_no_path() {
        check_error(Error::NoPath, &["patch", "-p"]);
    }

    #[test]
    fn long_in_file_error_no_path() {
        check_error(Error::NoVersion, &["--path", "patch"]);
    }

    #[test]
    fn mixed_flags_no_version_error() {
        check_error(Error::NoVersion, &["-fq", "--dry-run"]);
    }

    #[test]
    fn invalid_custom_versions_error() {
        let invalids = ["1.a.2", "0.0", "0", "v1.2.3", "2.3.4.5", "2.3.4no"];
        for v in invalids {
            check_error(Error::InvalidVersion(v.into()), &[v]);
        }
    }

    #[test]
    fn valid_custom_versions() {
        let valids = ["0.0.0", "0.1.2", "1.2.3", "1.2.3-beta"];
        for v in valids {
            check(Expected::new().version(v), &[v]);
        }
    }

    struct Expected<'a> {
        app: App<'a>,
    }

    macro_rules! builder_fn {
        ($field:ident) => {
            fn $field(mut self) -> Self {
                self.app.$field = true;
                self
            }
        };
    }

    impl<'a> Expected<'a> {
        fn new() -> Self {
            Self {
                app: App::default(),
            }
        }

        fn build(self) -> App<'a> {
            self.app
        }

        builder_fn! {help}
        builder_fn! {info}
        builder_fn! {quiet}
        builder_fn! {force}
        builder_fn! {dry_run}

        fn major(mut self) -> Self {
            self.app.version = Version::Major;
            self
        }

        fn minor(mut self) -> Self {
            self.app.version = Version::Minor;
            self
        }

        fn patch(mut self) -> Self {
            self.app.version = Version::Patch;
            self
        }

        fn version(mut self, v: &'a str) -> Self {
            self.app.version = Version::Custom(v);
            self
        }

        fn in_file(mut self, in_file: &'a str) -> Self {
            self.app.in_file = Some(Path::new(in_file));
            self
        }

        fn out_file(mut self, out_file: &'a str) -> Self {
            self.app.out_file = Some(Path::new(out_file));
            self
        }
    }
}

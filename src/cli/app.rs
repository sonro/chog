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

        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                app.handle_long_flag(arg, args.peek())?;
            } else if arg.starts_with('-') {
                app.handle_short_flag(arg, args.peek())?;
            }
            app.version = match arg {
                "major" => Version::Major,
                "minor" => Version::Minor,
                "patch" => Version::Patch,
                arg => custom_version_from_arg(arg)?,
            }
        }

        Ok(app)
    }

    fn handle_long_flag(&mut self, arg: &'a str, next: Option<&&'a str>) -> Result<(), Error> {
        match arg {
            "--help" => self.help = true,
            "--info" => self.info = true,
            "--quiet" => self.quiet = true,
            "--force" => self.force = true,
            "--dry-run" => self.dry_run = true,
            "--path" => self.in_file = Some(path_from_arg(next)),
            "--output" => self.out_file = Some(path_from_arg(next)),
            _ => return Err(Error::UnknownFlag(arg.into())),
        }
        Ok(())
    }

    fn handle_short_flag(&mut self, arg: &str, next: Option<&&'a str>) -> Result<(), Error> {
        for ch in arg.chars().skip(1) {
            match ch {
                'h' => {
                    self.help = true;
                    return Ok(());
                }
                'i' => self.info = true,
                'q' => self.quiet = true,
                'f' => self.force = true,
                'd' => self.dry_run = true,
                'p' => self.in_file = Some(path_from_arg(next)),
                'o' => self.out_file = Some(path_from_arg(next)),
                _ => return Err(Error::UnknownFlag(arg.into())),
            }
        }
        Ok(())
    }
}

fn path_from_arg<'a>(next_arg: Option<&&'a str>) -> &'a Path {
    match next_arg {
        Some(&arg) => Path::new(arg),
        None => unimplemented!(),
    }
}

fn custom_version_from_arg(arg: &str) -> Result<Version, Error> {
    for (i, part) in arg.split('.').enumerate() {
        match i {
            0 | 1 => {
                if part.chars().all(char::is_numeric) {
                    continue;
                }
            }
            2 => {
                if part.chars().all(char::is_numeric) {
                    continue;
                } else {
                    if let Some(patch) = part.split('-').next() {
                        if patch.chars().all(char::is_numeric) {
                            continue;
                        }
                    }
                    return Err(Error::InvalidVersion(arg.into()));
                }
            }
            _ => return Err(Error::InvalidVersion(arg.into())),
        }
    }
    Ok(Version::Custom(arg))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let app = App::new::<&str>(&[]).expect("no args shouldn't error");
        let expected = App {
            info: true,
            ..Default::default()
        };
        assert_eq!(expected, app);
    }
}

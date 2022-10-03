mod version;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Version<'a> {
    Major,
    Minor,
    Patch,
    Custom(&'a str),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidVersion(String);
